use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::task::JoinSet;

pub mod fstr;
pub mod const_option;
pub mod ptrarc;

pub enum AwaitOrClone<T: Clone> {
    InProgress(broadcast::Receiver<T>),
    Done(T),
}

impl<T: Clone> Clone for AwaitOrClone<T> {
    fn clone(&self) -> Self {
        match self {
            AwaitOrClone::InProgress(rec) => AwaitOrClone::InProgress(rec.resubscribe()),
            AwaitOrClone::Done(arc) => AwaitOrClone::Done(arc.clone()),
        }
    }
}

impl<T: Clone> AwaitOrClone<T> {
    pub async fn await_clone(&self) -> Result<T, RecvError> {
        let mut rec = match self {
            AwaitOrClone::InProgress(rec) => (*rec).resubscribe(),
            AwaitOrClone::Done(arc) => return Ok(arc.clone()),
        };
        rec.recv().await
    }
}

#[derive(Clone)]
pub struct AsyncMap<K: Hash, V: Clone>(Arc<tokio::sync::RwLock<HashMap<K, AwaitOrClone<V>>>>);
pub enum AsyncMapResult<V> {
    AlreadyDone(V),
    ItsOnTheWay(broadcast::Receiver<V>),
    Oops(RecvError),
}
impl<V: Clone> AsyncMapResult<V> {
    pub async fn get_or_panic(self) -> V {
        match self {
            AsyncMapResult::AlreadyDone(v) => v,
            AsyncMapResult::ItsOnTheWay(mut thingy) => thingy.recv().await.expect("AsyncMapResult recv error"),
            AsyncMapResult::Oops(err) => panic!("AsyncMapResult oopsie: {err:?}"),
            // _ => panic!()
        }
    }
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Clone + Send + Sync + 'static> AsyncMap<K, V> {
    pub fn new() -> Self {
        AsyncMap(Arc::new(tokio::sync::RwLock::new(HashMap::new())))
    }
    pub async fn get(&self, k: &K) -> Option<V> {
        let guard = self.0.read().await;
        let awaiter = guard.get(k).cloned()?;
        drop(guard);
        awaiter.await_clone().await.ok()
    }

    pub async fn get_or_create_or_panic<F: Future<Output = V> + Send + 'static>(&self, k: K, f: F) -> V {
        self.get_or_create(k, f).await.get_or_panic().await
    }
    pub async fn get_or_create<F: Future<Output = V> + Send + 'static>(&self, k: K, f: F) -> AsyncMapResult<V> {
        let read = self.0.read().await;
        match read.get(&k) {
            Some(existing) => {
                let result = existing.clone();
                drop(read);
                match result.await_clone().await {
                    Ok(v) => AsyncMapResult::AlreadyDone(v),
                    Err(err) => AsyncMapResult::Oops(err),
                }
            }
            None => {
                drop(read);
                let mut write = self.0.write().await;
                match write.entry(k.clone()) {
                    Entry::Occupied(occ) => {
                        let impl_started = occ.get().clone();
                        drop(write);
                        match impl_started.await_clone().await {
                            Ok(v) => AsyncMapResult::AlreadyDone(v),
                            Err(err) => AsyncMapResult::Oops(err),
                        }
                    }
                    Entry::Vacant(vac) => {
                        let (s, r) = broadcast::channel(1);
                        vac.insert(AwaitOrClone::InProgress(r.resubscribe()));
                        drop(write);
                        let self_ = self.clone();
                        tokio::spawn(async move {
                            let v = f.await;
                            let mut write = self_.0.write().await;
                            write
                                .entry(k.clone())
                                .and_modify(|it| *it = AwaitOrClone::Done(v.clone()))
                                .or_insert_with(|| AwaitOrClone::Done(v.clone()));
                            drop(write);
                            let _ = s.send(v);
                        });
                        AsyncMapResult::ItsOnTheWay(r)
                    }
                }
            }
        }
    }

    pub async fn to_vec(&self) -> Vec<V> {
        let mut result = vec![];
        for (_k, v) in self.0.read().await.iter() {
            let v = v.await_clone().await.expect("Shouldn't get receive error");
            result.push(v);
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstVec<T, const N: usize>([Option<T>; N]);

impl<T: Copy + Debug, const N: usize> ConstVec<T, N> {
    pub const fn new() -> Self {
        ConstVec([None; N])
    }

    pub const fn len(&self) -> usize {
        let mut s = 0;
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].is_some() {
                s += 1;
            }
            i += 1;
        }
        s
    }

    pub const fn push(&mut self, t: T) {
        let mut i = 0;
        while i < self.0.len() {
            let i_mut = &mut self.0[i];
            match i_mut {
                None => {
                    // T: Copy helps convince the compiler we are not running a destructor at
                    // compile time here
                    *i_mut = Some(t);
                    return;
                }
                Some(_) => {
                    // Continue to iterate until possibly overflow
                }
            }
            i += 1;
        }
        // Cannot format nicer error message in const evaluation
        panic!("ConstVec overflow")
    }

    pub const fn get(&self, index: usize) -> &T {
        match &self.0[index] {
            None => {
                // Cannot format nicer error message in const evaluation
                panic!("ConstVec get() index out of bounds")
            }
            Some(stuff) => return stuff,
        }
    }

    pub const fn get_mut(&mut self, index: usize) -> &mut T {
        match &mut self.0[index] {
            None => {
                panic!("ConstVec get_mut() index out of bounds")
            }
            Some(stuff) => return stuff,
        }
    }
}
impl<T: Copy, const N: usize> IntoIterator for ConstVec<T, N> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = vec![];
        for t in self.0.into_iter().filter_map(|it| it) {
            v.push(t);
        }
        v.into_iter()
    }
}
// TODO see if you can get the below item to work and remove the above item
// type ConstVecIterator<T> = FilterMap<std::vec::IntoIter<Option<T>>, fn(Option<T>) -> Option<T>>;
// impl<T: Copy, const N: usize> IntoIterator for ConstVec<T, N> {
// type Item = T;
// type IntoIter = ConstVecIterator<T>;
//
// fn into_iter(self) -> Self::IntoIter {
// This technique uses Copy to directly copy memory to the heap, instead of manually
// iterating to construct the Vec (and we want a Vec so that we don't have to worry about
// a type level size)
// let result = self.0.into_vec().into_iter().filter_map(|it| it);
// result
// }
// }
impl<T: Copy + PartialOrd, const N: usize> PartialOrd for ConstVec<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut i = 0;
        while i < N {
            let a = &self.0[i];
            let b = &other.0[i];
            i += 1;
            match (a, b) {
                (None, None) => return Some(Ordering::Equal),
                (Some(_), None) => return Some(Ordering::Greater),
                (None, Some(_)) => return Some(Ordering::Less),
                (Some(a), Some(b)) => match a.partial_cmp(b) {
                    None => return None,
                    Some(Ordering::Equal) => continue,
                    Some(o) => return Some(o),
                },
            }
        }
        return Some(Ordering::Equal);
    }
}
impl<T: Copy + Ord, const N: usize> Ord for ConstVec<T, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut i = 0;
        while i < N {
            let a = &self.0[i];
            let b = &other.0[i];
            i += 1;
            match (a, b) {
                (None, None) => return Ordering::Equal,
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (Some(a), Some(b)) => match a.cmp(b) {
                    Ordering::Equal => continue,
                    o => return o,
                },
            }
        }
        return Ordering::Equal;
    }
}

impl<T: Copy + Debug, const N: usize> Index<usize> for ConstVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}
impl<T: Copy + Debug, const N: usize> IndexMut<usize> for ConstVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}


#[async_trait]
pub trait CollectResults {
    async fn collect_results(self) -> anyhow::Result<()>;
}
#[async_trait]
impl CollectResults for JoinSet<anyhow::Result<()>> {
    async fn collect_results(mut self) -> anyhow::Result<()> {
        let mut errs = vec![];
        while let Some(result) = self.join_next().await {
            match result {
                Ok(Ok(())) => continue,
                Ok(Err(e)) => errs.push(e),
                Err(e) => errs.push(anyhow::Error::new(e)),
            }
        }
        return if errs.is_empty() {
            Ok(())
        } else {
            // Combine all errors into a single error
            let combined_error = errs.into_iter().map(|e| format!("{e:?}")).collect::<Vec<_>>().join(", ");
            Err(anyhow!(combined_error))
        };
    }
}

/// retain_mut for slices. Returns the final length
pub fn slice_retain_mut<T, F>(slice: &mut [T], mut f: F) -> usize
where
    F: FnMut(&mut T) -> bool,
{
    let mut i = 0;
    for j in 0..slice.len() {
        if f(&mut slice[j]) {
            slice.swap(i, j);
            i += 1;
        }
    }
    return i;
}


