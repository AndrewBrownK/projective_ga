
// The "Copy" ancestor for TraitImpls is experimental.
//  It might seem absurd and far too constraining at first, but actually it might be pretty
//  viable. Even when the implementing type isn't trivial, the ability to define static values
//  combined with the ability to impl traits for &'static Item makes this actually start
//  to look quite viable. I mean, scoped references are all we need for Copy, but the reason
//  we might want to use references to static items is so we can reference them inside
//  general_implementation without access to an enclosing scope (aside from static values).
//  In this way it is almost like the entire rs file becomes a script, defining variables
//  as statics and interdependent script snippets as trait implementations.

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_10: Copy + Send + Sync + 'static {
    type Output: TraitResultType;

    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_0_Args: TraitImpl_10 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }
    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Zero,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let cycle_detector_key = (trait_key.clone(), owner.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_10(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits10.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner.clone());
        let owner_clone = owner.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let fresh_variable_scope = Arc::new(Mutex::new(HashMap::new()));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, fresh_variable_scope, cycle_detector_clone);
            let trait_impl = t_self.general_implementation(builder, owner_clone.clone()).await?;
            trait_impl.into_trait10(owner_clone)
        };
        let the_impl = builder.registry.traits10.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits10_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let invocation_expression = <Self::Output as TraitResultType>::expr_10(trait_key.clone(), owner, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement>(&self, builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>, owner: MultiVector) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits10.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            // return builder.inline_by_copy_existing_10::<Self>(&trait_key, raw_impl);
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits10.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_11: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_1_Arg: TraitImpl_11 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::One,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_11(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits11.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self).await?;
            trait_impl.into_trait11(owner_class_clone)
        };
        let the_impl = builder.registry.traits11.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits11_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_11(trait_key.clone(), owner_param, mv_result);

        Some(invocation_expression)
    }
    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits11.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            // return builder.inline_by_copy_existing_11::<Self, _>(&trait_key, raw_impl, owner);
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits11.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_21: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: MultiVector,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2_Types_1_Arg: TraitImpl_21 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::One,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_class.clone()).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_21(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits21.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, other_class_clone.clone()).await?;
            trait_impl.into_trait21(owner_class_clone, other_class_clone.clone())
        };
        let the_impl = builder.registry.traits21.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits21_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_21(trait_key.clone(), owner_param, other_class, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type(), other.clone());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits21.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            // return builder.inline_by_copy_existing_21::<Self, _>(&trait_key, raw_impl, owner);
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr,
        other: MultiVector,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits21.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_22: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<MultiVector>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_2_Types_2_Args: TraitImpl_22 + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    type Other: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_class = other.expression_type();
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), Some(other_class.clone()));
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_22(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits22.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone(), other_class.clone());
        let owner_class_clone = owner_class.clone();
        let other_class_clone = other_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<MultiVector> = Variable {
                expr_type: other_class.clone(),
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            trait_impl.into_trait22(owner_class_clone.clone(), other_class_clone.clone())
        };
        let the_impl = builder.registry.traits22.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits22_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_multivector_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_22(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type(), other.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits22.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            // return builder.inline_by_copy_existing_22::<Self, _, _>(&trait_key, raw_impl, owner, other);
        }

        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // The correct/best way to do this is to use regular inline, and then substitute
        //  the variable declarations independently. This is because we support trait definitions
        //  using either invoke or inline, and so we don't know which we're going to get, but
        //  in any case a trait definition might define variables on its own, without necessarily
        //  doing so from an inlined invocation. So if it does that, then you have to replace
        //  variable declarations anyway. Regular inlining works by appending inner builders to
        //  an outer builder with finish_inline, so I think we can use a middle builder. The
        //  outer builder is whatever called deep_inline. The middle builder will do the variable
        //  inlining on the return value, and accumulate inner inlining. The inner builders do
        //  the regular inlinings which inline all the way down and use variables.

        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        // We do not implicitly declare the trait unless it is invoked by name.
        // Inlining will not trigger implicit declaration.
        // let the_def = b.registry.defs.traits22.get_or_create_or_panic(trait_key.clone(), async move { slf.def() }).await;
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_12f: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Float>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_2_Args_f32: TraitImpl_12f + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_12f(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits12f.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<Float> = Variable {
                expr_type: Float,
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            trait_impl.into_trait12f(owner_class_clone.clone())
        };
        let the_impl = builder.registry.traits12f.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits12f_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_float_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_12f(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits12f.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            //return builder.inline_by_copy_existing_12f::<Self, _, _>(&trait_key, raw_impl, owner, other);
        }

        let slf = self.clone();
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let slf = self.clone();
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitImpl_12i: Copy + Send + Sync + 'static {
    type Output: TraitResultType;
    async fn general_implementation<const AntiScalar: BasisElement>(
        self,
        builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
        slf: Variable<MultiVector>,
        other: Variable<Integer>,
    ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>>;
}

#[async_trait]
#[allow(non_camel_case_types)]
pub trait TraitDef_1_Type_2_Args_i32: TraitImpl_12i + ProvideTraitNames {
    type Owner: ClassesFromRegistry;
    fn general_documentation(&self) -> String {
        String::new()
    }

    fn def(&self) -> Arc<RawTraitDefinition> {
        Arc::new(RawTraitDefinition {
            documentation: self.general_documentation(),
            names: self.trait_names(),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Two,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        })
    }

    async fn invoke<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        let trait_key = self.trait_names().trait_key;
        let owner_class = owner.expression_type();
        let owner_param = owner;
        let other_param = other;
        let cycle_detector_key = (trait_key.clone(), owner_class.clone(), None);
        if builder.cycle_detector.contains(&cycle_detector_key) {
            let all_in_cycle = &builder.cycle_detector;
            panic!("Cycle detected at trait {trait_key:?}: {all_in_cycle:?}")
        } else {
            builder.cycle_detector.push(cycle_detector_key);
        }
        if builder.inline_dependencies {
            let return_as_var = self.inline(builder, owner_param, other_param).await?;
            return Some(<Self::Output as TraitResultType>::inlined_expr_12i(return_as_var));
        }

        let slf = self.clone();
        let the_def = builder.registry.defs.traits12f.expect_get_or_create(trait_key.clone(), async move { slf.def() }).await;
        let the_def_clone = the_def.clone();

        let impl_key = (trait_key.clone(), owner_class.clone());
        let owner_class_clone = owner_class.clone();
        let registry = builder.registry.clone();
        let cycle_detector_clone = builder.cycle_detector.clone();
        builder.cycle_detector.pop();
        let ga = builder.ga.clone();
        let mvs = builder.mvs.clone();
        let t_self = self.clone();
        let f = async move {
            // Create and register the implementation
            let mut fresh_variable_scope = HashMap::new();
            let declare_self = param_self();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_self));
            let declare_other = param_other();
            fresh_variable_scope.entry(declare_self.name.clone()).or_insert(Arc::downgrade(&declare_other));
            let builder = TraitImplBuilder::new(ga, mvs, the_def_clone, registry, false, Arc::new(Mutex::new(fresh_variable_scope)), cycle_detector_clone);
            let var_self: Variable<MultiVector> = Variable {
                expr_type: owner_class_clone.clone(),
                decl: declare_self,
            };
            let var_other: Variable<Integer> = Variable {
                expr_type: Integer,
                decl: declare_other,
            };
            let trait_impl = t_self.general_implementation(builder, var_self, var_other).await?;
            trait_impl.into_trait12i(owner_class_clone.clone())
        };
        let the_impl = builder.registry.traits12i.expect_get_or_create(impl_key.clone(), f).await?;
        let owner_type = ExpressionType::Class(owner_class.clone());
        let return_type = the_impl.return_expr.expression_type();

        TraitTypeConsensus::add_vote(&the_def.owner, owner_type, true);
        TraitTypeConsensus::add_vote(&the_def.output, return_type, owner_type == return_type);
        builder.trait_def.dependencies.lock().insert(trait_key);

        // We have an implementation. Great. Let's add the dependency.
        if let Some(_) = builder.traits12i_dependencies.insert(impl_key, the_impl.clone()) {
            // We already had the dependency. No problem.
        }
        let mv_result = match &the_impl.return_expr {
            AnyExpression::Class(mv) => Some(mv.expression_type()),
            _ => None,
        };
        let owner_param = extract_multivector_expr(owner_param);
        let other_param = extract_integer_expr(other_param);
        let invocation_expression = <Self::Output as TraitResultType>::expr_12i(trait_key.clone(), owner_param, other_param, mv_result);

        Some(invocation_expression)
    }

    async fn inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &mut TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<Self::Output>> {
        let trait_key = self.trait_names().trait_key;
        let impl_key = (trait_key.clone(), owner.expression_type());

        // Double Option/None: First is no impl attempted yet, Second is impl determined absent
        if let Some(Some(_raw_impl)) = builder.registry.traits12i.get(&impl_key).await {
            // Simplification can be expensive, reuse existing implementation
            // TODO enable and debug the panic
            // return builder.inline_by_copy_existing_12i::<Self, _, _>(&trait_key, raw_impl, owner, other);
        }

        let slf = self.clone();
        let the_def = slf.def();
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            builder.inline_dependencies,
            builder.variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        let var_name = trait_key.as_lower_snake();
        trait_impl.finish_inline(builder, var_name)
    }

    async fn deep_inline<const AntiScalar: BasisElement, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        builder: &TraitImplBuilder<AntiScalar, HasNotReturned>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<<Self::Output as TraitResultType>::Expr> {
        // The correct/best way to do this is to use regular inline, and then substitute
        //  the variable declarations independently. This is because we support trait definitions
        //  using either invoke or inline, and so we don't know which we're going to get, but
        //  in any case a trait definition might define variables on its own, without necessarily
        //  doing so from an inlined invocation. So if it does that, then you have to replace
        //  variable declarations anyway. Regular inlining works by appending inner builders to
        //  an outer builder with finish_inline, so I think we can use a middle builder. The
        //  outer builder is whatever called deep_inline. The middle builder will do the variable
        //  inlining on the return value, and accumulate inner inlining. The inner builders do
        //  the regular inlinings which inline all the way down and use variables.

        // let trait_key = self.trait_names().trait_key;
        let slf = self.clone();
        let the_def = slf.def();
        let variables = Arc::new(Mutex::new(HashMap::new()));
        let inner_builder = TraitImplBuilder::new(
            builder.ga.clone(),
            builder.mvs.clone(),
            the_def,
            builder.registry.clone(),
            true,
            variables.clone(),
            builder.cycle_detector.clone(),
        );
        let owner = inner_builder.coerce_variable("self", owner);
        let other = inner_builder.coerce_variable("other", other);
        let trait_impl = self.general_implementation(inner_builder, owner, other).await?;
        trait_impl.finish_deep_inline()
    }
}