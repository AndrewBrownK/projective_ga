
pub struct DebugTrait<T>(pub T);
#[async_trait]
pub(crate) trait Debug10 {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}
#[async_trait]
impl<T: TraitDef_1_Type_0_Args> Debug10 for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, self.0.def(), TraitImplRegistry::new(), false, Arc::new(Mutex::new(HashMap::new())), vec![]);
        // TODO add tracing in general implementation too.
        let b = self.0.general_implementation(b, mv_a).await?;
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        b.into_trait10(mv_a)
    }
}

#[async_trait]
pub(crate) trait Debug11 {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}

#[async_trait]
impl<T: TraitDef_1_Type_1_Arg> Debug11 for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let mut variables = HashMap::new();
        let declare_self = param_self();
        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
        let var_self: Variable<MultiVector> = Variable { expr_type: mv_a.clone(), decl: declare_self };
        let def = self.0.def();
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, def, TraitImplRegistry::new(), false, Arc::new(Mutex::new(variables)), vec![]);
        // TODO add tracing in general implementation too.
        let b = self.0.general_implementation(b, var_self).await?;
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        b.into_trait11(mv_a)
    }
}

#[async_trait]
pub(crate) trait Debug21 {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        mv_b: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}
#[async_trait]
impl<T: TraitDef_2_Types_1_Arg> Debug21 for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        mv_b: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let mv_b = MultiVector::from(mv_b);
        let mut variables = HashMap::new();
        let declare_self = param_self();
        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
        let var_self: Variable<MultiVector> = Variable { expr_type: mv_a.clone(), decl: declare_self };
        let def = self.0.def();
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, def, TraitImplRegistry::new(), false, Arc::new(Mutex::new(variables)), vec![]);
        // TODO add tracing in general implementation too.
        let b = self.0.general_implementation(b, var_self, mv_b).await?;
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        b.into_trait21(mv_a, mv_b)
    }
}

#[async_trait]
pub(crate) trait Debug22 {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        mv_b: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}
#[async_trait]
impl<T: TraitDef_2_Types_2_Args> Debug22 for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        mv_b: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let mv_b = MultiVector::from(mv_b);
        let mut variables = HashMap::new();

        let declare_self = param_self();
        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
        let var_self: Variable<MultiVector> = Variable { expr_type: mv_a.clone(), decl: declare_self };
        let declare_other = param_other();
        variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
        let var_other: Variable<MultiVector> = Variable { expr_type: mv_b.clone(), decl: declare_other };

        let def = self.0.def();
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, def, TraitImplRegistry::new(), false, Arc::new(Mutex::new(variables)), vec![]);
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        let b = self.0.general_implementation(b, var_self, var_other).await?;
        b.into_trait22(mv_a, mv_b)
    }
}

#[async_trait]
pub(crate) trait Debug12f {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_f32> Debug12f for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let mut variables = HashMap::new();

        let declare_self = param_self();
        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
        let var_self: Variable<MultiVector> = Variable { expr_type: mv_a.clone(), decl: declare_self };
        let declare_other = param_other();
        variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
        let var_other: Variable<Float> = Variable { expr_type: Float, decl: declare_other };

        let def = self.0.def();
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, def, TraitImplRegistry::new(), false, Arc::new(Mutex::new(variables)), vec![]);
        // TODO add tracing in general implementation too.
        let b = self.0.general_implementation(b, var_self, var_other).await?;
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        b.into_trait12f(mv_a)
    }
}

#[async_trait]
pub(crate) trait Debug12i {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>>;
}
#[async_trait]
impl<T: TraitDef_1_Type_2_Args_i32> Debug12i for DebugTrait<T> {
    async fn trace_implementation<const AntiScalar: BasisElement>(
        &self,
        filter: Level,
        mv_repo: Arc<MultiVecRepository<AntiScalar>>,
        mv_a: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
    ) -> Option<Arc<RawTraitImplementation>> {
        let mv_a = MultiVector::from(mv_a);
        let mut variables = HashMap::new();

        let declare_self = param_self();
        variables.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
        let var_self: Variable<MultiVector> = Variable { expr_type: mv_a.clone(), decl: declare_self };
        let declare_other = param_other();
        variables.entry(declare_other.name.clone()).or_insert(Arc::downgrade(&declare_other));
        let var_other: Variable<Integer> = Variable { expr_type: Integer, decl: declare_other };

        let def = self.0.def();
        let b = TraitImplBuilder::new(mv_repo.ga(), mv_repo, def, TraitImplRegistry::new(), false, Arc::new(Mutex::new(variables)), vec![]);
        // TODO add tracing in general implementation too.
        let b = self.0.general_implementation(b, var_self, var_other).await?;
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .event_format(DebuggableCopyPasta::new())
            .init();
        b.into_trait12i(mv_a)
    }
}
