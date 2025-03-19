use std::process::abort;
use colored::Colorize;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct TraitDefRegistry {
    traits10: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits11: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits12f: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits12i: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits21: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
    traits22: AsyncMap<TraitKey, Arc<RawTraitDefinition>>,
}
impl TraitDefRegistry {
    fn new() -> Self {
        TraitDefRegistry {
            traits10: AsyncMap::new(),
            traits11: AsyncMap::new(),
            traits12f: AsyncMap::new(),
            traits12i: AsyncMap::new(),
            traits21: AsyncMap::new(),
            traits22: AsyncMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct TraitImplRegistry {
    defs: TraitDefRegistry,
    // The use of options is to help short circuit the forgoing of implementations
    // that we've already determined cannot exist. Basically, the map might get fat with
    // None entries, but should save us a bit of compute cycles from repeatedly attempting
    // and failing to generate some trait implementations.
    traits10: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits11: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits12i: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits12f: AsyncMap<(TraitKey, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits21: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,
    traits22: AsyncMap<(TraitKey, MultiVector, MultiVector), Option<Arc<RawTraitImplementation>>>,

    has_set_operators: Arc<Mutex<BTreeSet<Ops>>>,
    pub infix_trick: Arc<Mutex<Option<BinaryOps>>>,
}

impl TraitImplRegistry {
    pub fn new() -> Self {
        TraitImplRegistry {
            defs: TraitDefRegistry::new(),
            traits10: AsyncMap::new(),
            traits11: AsyncMap::new(),
            traits12i: AsyncMap::new(),
            traits12f: AsyncMap::new(),
            traits21: AsyncMap::new(),
            traits22: AsyncMap::new(),
            has_set_operators: Arc::new(Mutex::new(BTreeSet::new())),
            infix_trick: Arc::new(Mutex::new(None)),
        }
    }

    // TODO allow 12f and 12i as well
    pub fn set_binary_operator<TD: TraitDef_2_Types_2_Args, const AntiScalar: BasisElement>(
        &self, repo: Arc<MultiVecRepository<AntiScalar>>,
        op: BinaryOps,
        td: TD
    ) {
        let mut set_ops = self.has_set_operators.lock();
        let op_key = op.rust_trait_name();
        let op = Ops::Binary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        let slf = self.clone();
        let multi_progress = Arc::new(MultiProgress::new());
        rt.block_on(async move {
            let overall_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(0).with_finish(ProgressFinish::AndLeave)));
            overall_pb.set_style(progress_style());
            overall_pb.set_message(format!("Operator Overloading: {}", op.rust_operator()));

            // Check if the trait has already been registered.
            // If it has, then the operator will overtly delegate to it.
            // If it hasn't, then we will inline the trait, so that it doesn't have to be declared separately.
            let orig_key = td.def().names.trait_key;
            let orig_td = tdr.traits22.get(&orig_key).await;
            let key = if orig_td.is_none() {
                let td = OvertDelegate::new(op_key, InlineOnly::new(orig_key.final_name, td));
                RegisterTrait(td).register::<AntiScalar>(slf, repo, multi_progress, Some(overall_pb.clone())).await;
                td.trait_names().trait_key
            } else {
                let td = OvertDelegate::new(op_key, td);
                RegisterTrait(td).register::<AntiScalar>(slf, repo, multi_progress, Some(overall_pb.clone())).await;
                td.trait_names().trait_key
            };
            let def = tdr.traits22.get(&key).await.expect("Created during registration");
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!(
                    "Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}"
                );
            }
            *the_op = Some(op);
            overall_pb.finish_and_clear();
        });
    }

    pub fn set_unary_operator<TD: TraitDef_1_Type_1_Arg, const AntiScalar: BasisElement>(
        &self, repo: Arc<MultiVecRepository<AntiScalar>>,
        op: UnaryOps,
        td: TD
    ) {
        let mut set_ops = self.has_set_operators.lock();
        let op_key = op.rust_trait_name();
        let op = Ops::Unary(op);
        if set_ops.contains(&op) {
            panic!("There was an attempt to set an operator's trait more than once: {op:?}")
        }
        set_ops.insert(op);
        drop(set_ops);

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let tdr = self.defs.clone();
        let slf = self.clone();
        let multi_progress = Arc::new(MultiProgress::new());
        rt.block_on(async move {
            let overall_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(0).with_finish(ProgressFinish::AndLeave)));
            overall_pb.set_style(progress_style());
            overall_pb.set_message(format!("Operator Overloading: {}", op.rust_operator()));

            // Check if the trait has already been registered.
            // If it has, then the operator will overtly delegate to it.
            // If it hasn't, then we will inline the trait, so that it doesn't have to be declared separately.
            let orig_key = td.def().names.trait_key;
            let orig_td = tdr.traits11.get(&orig_key).await;
            let key = if orig_td.is_none() {
                let td = OvertDelegate::new(op_key, InlineOnly::new(orig_key.final_name, td));
                RegisterTrait(td).register::<AntiScalar>(slf, repo, multi_progress, Some(overall_pb.clone())).await;
                td.trait_names().trait_key
            } else {
                let td = OvertDelegate::new(op_key, td);
                RegisterTrait(td).register::<AntiScalar>(slf, repo, multi_progress, Some(overall_pb.clone())).await;
                td.trait_names().trait_key
            };
            let def = tdr.traits11.get(&key).await.expect("Created during registration");
            let mut the_op = def.op.lock();
            if the_op.is_some() {
                // Shouldn't really happen because of previous panic/check in this function
                panic!(
                    "Somehow an operator was set twice, we should try to prevent this \
                    earlier than when we enter async machinery. {op:?}"
                );
            }
            *the_op = Some(op);
            overall_pb.finish_and_clear();
        });
    }

    pub fn generate_infix_trick(&self, op: BinaryOps) {
        let mut trick = self.infix_trick.lock();
        if trick.is_some() {
            panic!(
                "Do not set the infix trick more than once. Just decide what you want it to be \
                once and leave it."
            )
        }
        *trick = Some(op);
    }

    pub(crate) async fn get_defs(&self) -> Vec<Arc<RawTraitDefinition>> {
        let mut v = vec![];
        for dep in self.defs.traits10.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits11.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits12f.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits12i.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits21.to_vec().await {
            v.push(dep);
        }
        for dep in self.defs.traits22.to_vec().await {
            v.push(dep);
        }
        v
    }

    pub(crate) async fn get_impls(&self) -> Vec<Arc<RawTraitImplementation>> {
        let mut v = vec![];
        for dep in self.traits10.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits11.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits12f.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits12i.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits21.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        for dep in self.traits22.to_vec().await {
            if let Some(dep) = dep {
                v.push(dep);
            }
        }
        v
    }

    pub fn finish(self) -> Arc<Self> {
        Arc::new(self)
    }
}

pub fn progress_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .expect("Template should be good or you gotta fix it")
        .progress_chars("#>-")
}
pub fn tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("Tokio should work")
}
pub fn tokio_joinset<T>() -> JoinSet<T> {
    JoinSet::new()
}
pub fn indicatif_multi_progress() -> Arc<MultiProgress> {
    Arc::new(MultiProgress::new())
}
pub fn indicatif_progress_bar(s: u64) -> indicatif::ProgressBar {
    indicatif::ProgressBar::new(s)
}
pub fn indicatif_and_leave() -> ProgressFinish {
    ProgressFinish::AndLeave
}

pub struct RegisterTrait<T>(pub T);


// TODO make the toggling of progress bars easier but not generics params because that's dumb


#[async_trait]
pub trait Register10 {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tr: TraitImplRegistry,
        mvs: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}
#[async_trait]
impl<T: TraitDef_1_Type_0_Args> Register10 for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits10.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }

        let mut qty_done = 0;
        let update_period = 50;
        for mv_a in mv_repo.all_classes() {
            let mv_a = MultiVector::from(mv_a);
            let tir_2 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let the_impl = tir
                .traits10
                .expect_get_or_create((trait_key, mv_a), async move {
                    let variables = Arc::new(Mutex::new(HashMap::new()));
                    let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_2, tir_2, false, variables, vec![]);
                    let handle = tokio::task::spawn(async move {
                        self.0.general_implementation(b, mv_a.clone()).await?.into_trait10(mv_a)
                    });
                    match handle.await {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!(
                                "\n{} Attempt debugging with the following snippet:\n\
                                DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}).await;\n",
                                "Error while registering trait.".red(),
                                trait_key.as_upper_camel(), mv_a.name()
                            );
                            abort()
                        }
                    }
                })
                .await;
            if progress_bars && let Some(pb) = &pb {
                qty_done += 1;
                pb.inc(1);
                if qty_done % update_period == 0 && let Some(op) = &overall_progress {
                    op.inc(update_period);
                }
            }
            let Some(the_impl) = the_impl else { continue };
            let owner_type = ExpressionType::Class(mv_a.clone());
            let return_type = the_impl.return_expr.expression_type();
            TraitTypeConsensus::add_vote(&def.owner, owner_type, true);
            TraitTypeConsensus::add_vote(&def.output, return_type, owner_type == return_type);
        }
        if progress_bars && let (Some(pb), Some(op)) = (&pb, &overall_progress) {
            op.inc(qty % update_period);
            pb.finish_and_clear();
        }
    }
}

#[async_trait]
pub trait Register11 {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}

#[async_trait]
impl<T: TraitDef_1_Type_1_Arg> Register11 for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits11.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }

        let mut qty_done = 0;
        let update_period = 50;
        for mv_a in mv_repo.all_classes() {
            let mv_a = MultiVector::from(mv_a);
            let tir_2 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let the_impl = tir
                .traits11
                .expect_get_or_create((trait_key, mv_a), async move {
                    let mut variables = HashMap::new();
                    let declare_self = param_self();
                    variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                    let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_2, tir_2, false, Arc::new(Mutex::new(variables)), vec![]);
                    let var_self: Variable<MultiVector> = Variable {
                        expr_type: mv_a.clone(),
                        decl: declare_self,
                    };
                    let handle = tokio::task::spawn(async move {
                        self.0.general_implementation(b, var_self).await?.into_trait11(mv_a)
                    });
                    match handle.await {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!(
                                "\n{} Attempt debugging with the following snippet:\n\
                                DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}).await;\n",
                                "Error while registering trait.".red(),
                                trait_key.as_upper_camel(), mv_a.name()
                            );
                            abort()
                        }
                    }
                })
                .await;
            if progress_bars && let Some(pb) = &pb {
                qty_done += 1;
                pb.inc(1);
                if qty_done % update_period == 0 && let Some(op) = &overall_progress{
                    op.inc(update_period);
                }
            }
            let Some(the_impl) = the_impl else { continue };
            let owner_type = ExpressionType::Class(mv_a.clone());
            let return_type = the_impl.return_expr.expression_type();
            TraitTypeConsensus::add_vote(&def.owner, owner_type, true);
            TraitTypeConsensus::add_vote(&def.output, return_type, owner_type == return_type);
        }
        if progress_bars && let (Some(pb), Some(op)) = (&pb, &overall_progress) {
            op.inc(qty % update_period);
            pb.finish_and_clear();
        }
    }
}

#[async_trait]
pub trait Register21 {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}
#[async_trait]
impl<T: TraitDef_2_Types_1_Arg> Register21 for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits21.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let big_qty = qty * qty;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(big_qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(big_qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }

        let update_period = 50;
        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let mv_repo_2 = mv_repo.clone();
            let tir_2 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let overall_progress_2 = overall_progress.clone();
            let pb_2 = pb.clone();
            js.spawn(async move {
                let mut qty_done = 0;
                let mv_a = MultiVector::from(mv_a);
                for mv_b in mv_repo_2.all_classes() {
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let tir_4 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let ga_3 = ga_2.clone();
                    let mv_repo_3 = mv_repo_2.clone();
                    let the_impl = tir_3
                        .traits21
                        .expect_get_or_create((trait_key, mv_a, mv_b), async move {
                            let mut variables = HashMap::new();
                            let declare_self = param_self();
                            variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                            let b = TraitImplBuilder::new(ga_3, mv_repo_3, def_3, tir_4, false, Arc::new(Mutex::new(variables)), vec![]);
                            let var_self: Variable<MultiVector> = Variable {
                                expr_type: mv_a.clone(),
                                decl: declare_self,
                            };
                            let handle = tokio::task::spawn(async move {
                                self.0.general_implementation(b, var_self, mv_b.clone()).await?.into_trait21(mv_a, mv_b)
                            });
                            match handle.await {
                                Ok(s) => s,
                                Err(_) => {
                                    eprintln!(
                                        "\n{} Attempt debugging with the following snippet:\n\
                                        DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}, &{}).await;\n",
                                        "Error while registering trait.".red(),
                                        trait_key.as_upper_camel(), mv_a.name(), mv_b.name()
                                    );
                                    abort()
                                }
                            }
                        })
                        .await;
                    if progress_bars && let Some(pb) = &pb_2 {
                        pb.inc(1);
                        qty_done += 1;
                        if qty_done % update_period == 0 && let Some(op) = &overall_progress_2 {
                            op.inc(update_period);
                        }
                    }
                    let Some(the_impl) = the_impl else { continue };
                    let owner_type = ExpressionType::Class(mv_a.clone());
                    let return_type = the_impl.return_expr.expression_type();
                    TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                    TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
                }
                if progress_bars && let Some(op) = &overall_progress_2 {
                    op.inc(qty % update_period);
                }
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        if progress_bars && let Some(pb) = &pb {
            pb.finish_and_clear();
        }
    }
}

#[async_trait]
pub trait Register22 {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}
#[async_trait]
impl<T: TraitDef_2_Types_2_Args> Register22 for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits22.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let big_qty = qty * qty;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(big_qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(big_qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }

        let update_period = 50;
        let mut js = JoinSet::new();
        for mv_a in mv_repo.all_classes() {
            let mv_repo_2 = mv_repo.clone();
            let tir_2 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let overall_progress_2 = overall_progress.clone();
            let pb_2 = pb.clone();
            js.spawn(async move {
                let mut qty_done = 0;
                let mv_a = MultiVector::from(mv_a);
                for mv_b in mv_repo_2.all_classes() {
                    let mv_b = MultiVector::from(mv_b);
                    let tir_3 = tir_2.clone();
                    let tir_4 = tir_2.clone();
                    let def_3 = def_2.clone();
                    let ga_3 = ga_2.clone();
                    let mv_repo_3 = mv_repo_2.clone();
                    let the_impl = tir_3
                        .traits22
                        .expect_get_or_create((trait_key, mv_a, mv_b), async move {
                            let mut variables = HashMap::new();
                            let declare_self = param_self();
                            variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                            let declare_other = param_other();
                            variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
                            let b = TraitImplBuilder::new(ga_3, mv_repo_3, def_3, tir_4, false, Arc::new(Mutex::new(variables)), vec![]);
                            let var_self: Variable<MultiVector> = Variable {
                                expr_type: mv_a.clone(),
                                decl: declare_self,
                            };
                            let var_other: Variable<MultiVector> = Variable {
                                expr_type: mv_b.clone(),
                                decl: declare_other,
                            };
                            let handle = tokio::task::spawn(async move {
                                self.0.general_implementation(b, var_self, var_other).await?.into_trait22(mv_a, mv_b)
                            });
                            match handle.await {
                                Ok(s) => s,
                                Err(_) => {
                                    eprintln!(
                                        "\n{} Attempt debugging with the following snippet:\n\
                                        DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}, &{}).await;\n",
                                        "Error while registering trait.".red(),
                                        trait_key.as_upper_camel(), mv_a.name(), mv_b.name()
                                    );
                                    abort()
                                }
                            }
                        })
                        .await;
                    if progress_bars && let Some(pb) = &pb_2 {
                        pb.inc(1);
                        qty_done += 1;
                        if qty_done % update_period == 0 && let Some(op) = &overall_progress_2 {
                            op.inc(update_period);
                        }
                    }
                    let Some(the_impl) = the_impl else { continue };
                    let owner_type = ExpressionType::Class(mv_a.clone());
                    let return_type = the_impl.return_expr.expression_type();
                    TraitTypeConsensus::add_vote(&def_2.owner, owner_type, true);
                    TraitTypeConsensus::add_vote(&def_2.output, return_type, owner_type == return_type);
                }
                if progress_bars && let Some(op) = &overall_progress_2 {
                    op.inc(qty % update_period);
                }
            });
        }
        while let Some(result) = js.join_next().await {
            let _: () = result.expect("async machinery should work");
        }
        if progress_bars && let Some(pb) = &pb {
            pb.finish_and_clear();
        }
    }
}

#[async_trait]
pub trait Register12f {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_f32> Register12f for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits12f.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }
        let mut qty_done = 0;
        let update_period = 50;

        for mv_a in mv_repo.all_classes() {
            let mv_a = MultiVector::from(mv_a);
            let tir_2 = tir.clone();
            let tir_3 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let the_impl = tir_2
                .traits12f
                .expect_get_or_create((trait_key, mv_a), async move {
                    let mut variables = HashMap::new();
                    let declare_self = param_self();
                    variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                    let declare_other = param_other();
                    variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
                    let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_2, tir_3, false, Arc::new(Mutex::new(variables)), vec![]);
                    let var_self: Variable<MultiVector> = Variable {
                        expr_type: mv_a.clone(),
                        decl: declare_self,
                    };
                    let var_other: Variable<Float> = Variable {
                        expr_type: Float,
                        decl: declare_other,
                    };
                    let handle = tokio::task::spawn(async move {
                        self.0.general_implementation(b, var_self, var_other).await?.into_trait12f(mv_a)
                    });
                    match handle.await {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!(
                                "\n{} Attempt debugging with the following snippet:\n\
                                        DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}).await;\n",
                                "Error while registering trait.".red(),
                                trait_key.as_upper_camel(), mv_a.name()
                            );
                            abort()
                        }
                    }
                })
                .await;
            if progress_bars && let Some(pb) = &pb {
                qty_done += 1;
                pb.inc(1);
                if qty_done % update_period == 0 && let Some(op) = &overall_progress {
                    op.inc(update_period);
                }
            }
            let Some(the_impl) = the_impl else { continue };
            let owner_type = ExpressionType::Class(mv_a.clone());
            let return_type = the_impl.return_expr.expression_type();
            TraitTypeConsensus::add_vote(&def.owner, owner_type, true);
            TraitTypeConsensus::add_vote(&def.output, return_type, owner_type == return_type);
        }
        if progress_bars && let (Some(pb), Some(op)) = (&pb, &overall_progress) {
            op.inc(qty % update_period);
            pb.finish_and_clear();
        }
    }
}

#[async_trait]
pub trait Register12i {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    );
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_i32> Register12i for RegisterTrait<T> {
    async fn register<const AntiScalar: BasisElement>(
        self,
        tir: TraitImplRegistry,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        progress: Arc<MultiProgress>,
        overall_progress: Option<Arc<indicatif::ProgressBar>>,
    ) {
        let progress_bars = overall_progress.is_some();
        let ga = mv_repo.ga();
        let trait_key = self.0.trait_names().trait_key;
        let def = tir.defs.traits12i.expect_get_or_create(trait_key.clone(), async move { self.0.def() }).await;

        let qty = mv_repo.qty_classes() as u64;
        let qty = qty * qty;
        let mut pb = None;
        if progress_bars && let Some(op) = &overall_progress {
            op.inc_length(qty);
            let prog_bar = Arc::new(progress.add(indicatif::ProgressBar::new(qty)));
            prog_bar.set_style(progress_style());
            let n = trait_key.as_upper_camel();
            prog_bar.set_message(format!("AST: {n}"));
            pb = Some(prog_bar);
        }

        let mut qty_done = 0;
        let update_period = 50;
        for mv_a in mv_repo.all_classes() {
            let mv_a = MultiVector::from(mv_a);
            let tir_2 = tir.clone();
            let tir_3 = tir.clone();
            let def_2 = def.clone();
            let ga_2 = ga.clone();
            let mv_repo_2 = mv_repo.clone();
            let the_impl = tir_2
                .traits12i
                .expect_get_or_create((trait_key, mv_a), async move {
                    let mut variables = HashMap::new();
                    let declare_self = param_self();
                    variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
                    let declare_other = param_other();
                    variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
                    let b = TraitImplBuilder::new(ga_2, mv_repo_2, def_2, tir_3, false, Arc::new(Mutex::new(variables)), vec![]);
                    let var_self: Variable<MultiVector> = Variable {
                        expr_type: mv_a.clone(),
                        decl: declare_self,
                    };
                    let var_other: Variable<Integer> = Variable {
                        expr_type: Integer,
                        decl: declare_other,
                    };
                    let handle = tokio::task::spawn(async move {
                        self.0.general_implementation(b, var_self, var_other).await?.into_trait12i(mv_a)
                    });
                    match handle.await {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!(
                                "\n{} Attempt debugging with the following snippet:\n\
                                        DebugTrait({}).trace_implementation(Level::TRACE, repo, &{}).await;\n",
                                "Error while registering trait.".red(),
                                trait_key.as_upper_camel(), mv_a.name()
                            );
                            abort()
                        }
                    }
                })
                .await;
            if progress_bars && let Some(pb) = &pb {
                qty_done += 1;
                pb.inc(1);
                if qty_done % update_period == 0 && let Some(op) = &overall_progress {
                    op.inc(update_period);
                }
            }
            let Some(the_impl) = the_impl else { continue };
            let owner_type = ExpressionType::Class(mv_a.clone());
            let return_type = the_impl.return_expr.expression_type();
            TraitTypeConsensus::add_vote(&def.owner, owner_type, true);
            TraitTypeConsensus::add_vote(&def.output, return_type, owner_type == return_type);
        }
        if progress_bars && let (Some(pb), Some(op)) = (&pb, &overall_progress) {
            op.inc(qty % update_period);
            pb.finish_and_clear();
        }
    }
}
