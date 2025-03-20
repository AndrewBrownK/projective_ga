use crate::ast::expressions::DebugExpression;

pub struct HasNotReturned;

pub(crate) fn param_self() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("self".to_string(), 0),
        expr: None,
        force_inline: Arc::new(AtomicBool::new(false)),
    })
}
pub(crate) fn param_other() -> Arc<RawVariableDeclaration> {
    Arc::new(RawVariableDeclaration {
        comment: None,
        name: ("other".to_string(), 0),
        expr: None,
        force_inline: Arc::new(AtomicBool::new(false)),
    })
}

#[derive(Clone)]
pub enum CommentOrVariableDeclaration {
    Comment(Cow<'static, String>),
    VarDec(Weak<RawVariableDeclaration>),
}
impl Debug for CommentOrVariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentOrVariableDeclaration::Comment(s) => {
                write!(f, "// {s}")
            }
            CommentOrVariableDeclaration::VarDec(v) => {
                let Some(v) = v.upgrade() else {
                    return write!(f, "// This comment is an unused variable that will get removed");
                };
                write!(f, "let ")?;
                let derived_name = match &v.name {
                    (n, 0) => {
                        if n.as_str() == "self" {
                            write!(f, "slf")?;
                            "slf".to_string()
                        } else {
                            write!(f, "{n}")?;
                            n.clone()
                        }
                    }
                    (n, i) => {
                        write!(f, "{n}_{}", i + 1)?;
                        format!("{n}_{}", i + 1)
                    }
                };
                write!(f, " = ")?;
                match &v.expr {
                    None => write!(f, "todo!(\"variable has no backing\")")?,
                    Some(d) => {
                        let d = d.read();
                        let i = v.force_inline.load(Acquire);
                        match (i, &*d) {
                            (false, AnyExpression::Int(e)) => write!(f, "int_var(\"{derived_name}\", Some({:?}))", DebugExpression::new(true, e))?,
                            (false, AnyExpression::Float(e)) => write!(f, "float_var(\"{derived_name}\", Some({:?}))", DebugExpression::new(true, e))?,
                            (false, AnyExpression::Vec2(e)) => write!(f, "vec2_var(\"{derived_name}\", Some({:?}))", DebugExpression::new(true, e))?,
                            (false, AnyExpression::Vec3(e)) => write!(f, "vec3_var(\"{derived_name}\", Some({:?}))", DebugExpression::new(true, e))?,
                            (false, AnyExpression::Vec4(e)) => write!(f, "vec4_var(\"{derived_name}\", Some({:?}))", DebugExpression::new(true, e))?,
                            (false, AnyExpression::Class(e)) => {
                                let mv = e.mv_class.name();
                                write!(f, "multivec_var(\"{derived_name}\", &{mv}, Some({:?}))", DebugExpression::new(true, e))?
                            },
                            (true, AnyExpression::Int(e)) => write!(f, "int_var_will_be_inlined(\"{derived_name}\", {:?})", DebugExpression::new(true, e))?,
                            (true, AnyExpression::Float(e)) => write!(f, "float_var_will_be_inlined(\"{derived_name}\", {:?})", DebugExpression::new(true, e))?,
                            (true, AnyExpression::Vec2(e)) => write!(f, "vec2_var_will_be_inlined(\"{derived_name}\", {:?})", DebugExpression::new(true, e))?,
                            (true, AnyExpression::Vec3(e)) => write!(f, "vec3_var_will_be_inlined(\"{derived_name}\", {:?})", DebugExpression::new(true, e))?,
                            (true, AnyExpression::Vec4(e)) => write!(f, "vec4_var_will_be_inlined(\"{derived_name}\", {:?})", DebugExpression::new(true, e))?,
                            (true, AnyExpression::Class(e)) => {
                                let mv = e.mv_class.name();
                                write!(f, "multivec_var_will_be_inlined(\"{derived_name}\", &{mv}, {:?})", DebugExpression::new(true, e))?
                            },
                        }
                    }
                }
                write!(f, ";")
            }
        }
    }
}
impl CommentOrVariableDeclaration {
    fn needs_more_inlining(&self) -> bool {
        let CommentOrVariableDeclaration::VarDec(rvd) = self else { return false; };
        rvd.strong_count() <= 1
    }
}


pub struct TraitImplBuilder<const AntiScalar: BasisElement, ReturnType> {
    pub ga: Arc<GeometricAlgebra<AntiScalar>>,
    pub mvs: Arc<MultiVecRepository<AntiScalar>>,
    registry: TraitImplRegistry,
    pub(crate) trait_def: Arc<RawTraitDefinition>,
    inline_dependencies: bool,
    pub(crate) is_deep_inlining: bool,
    specialized: bool,

    cycle_detector: Vec<(TraitKey, MultiVector, Option<MultiVector>)>,
    pub(crate) multivector_dependencies: Mutex<HashSet<MultiVector>>,
    traits10_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits11_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits12f_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits12i_dependencies: HashMap<(TraitKey, MultiVector), Arc<RawTraitImplementation>>,
    traits21_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    traits22_dependencies: HashMap<(TraitKey, MultiVector, MultiVector), Arc<RawTraitImplementation>>,
    wanted_multi_vecs: Mutex<HashSet<BTreeSet<BasisSignature>>>,

    variables: Arc<Mutex<HashMap<(String, usize), Weak<RawVariableDeclaration>>>>,
    lines: Mutex<Vec<CommentOrVariableDeclaration>>,
    return_comment: Option<String>,
    return_expr: Option<AnyExpression>,
    return_type: ReturnType,
}

fn make_var_name_unique<RVD>(variables: &Mutex<HashMap<(String, usize), RVD>>, var_name: String) -> (String, usize) {
    let mut key = (var_name.to_string(), 0);
    let vars = variables.lock();
    while vars.contains_key(&mut key) {
        key.1 += 1;
    }
    key
}

impl<const AntiScalar: BasisElement> TraitImplBuilder<AntiScalar, HasNotReturned> {
    fn new(
        ga: Arc<GeometricAlgebra<AntiScalar>>,
        mvs: Arc<MultiVecRepository<AntiScalar>>,
        trait_def: Arc<RawTraitDefinition>,
        registry: TraitImplRegistry,
        inline_dependencies: bool,
        variables: Arc<Mutex<HashMap<(String, usize), Weak<RawVariableDeclaration>>>>,
        cycle_detector: Vec<(TraitKey, MultiVector, Option<MultiVector>)>,
    ) -> Self {
        // `self` vs `this` compatibility across rust and slang.
        let mut vars = variables.lock();
        let self_key = ("self".to_string(), 0);
        let this_key = ("this".to_string(), 0);
        if vars.contains_key(&self_key) && !vars.contains_key(&this_key) {
            vars.insert(this_key, Weak::new());
        }
        drop(vars);

        TraitImplBuilder {
            ga,
            mvs,
            registry,
            trait_def,
            inline_dependencies,
            is_deep_inlining: false,
            specialized: false,
            cycle_detector,
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
            traits12f_dependencies: Default::default(),
            traits12i_dependencies: Default::default(),
            traits21_dependencies: Default::default(),
            traits22_dependencies: Default::default(),
            wanted_multi_vecs: Default::default(),
            variables,
            lines: Mutex::new(vec![]),
            return_comment: None,
            return_expr: None,
            return_type: HasNotReturned,
        }
    }

    // TODO put a wrapper type on this maybe... or rename it to be independent of trait building.
    pub fn new_sandbox(ga: Arc<GeometricAlgebra<AntiScalar>>, mvs: Arc<MultiVecRepository<AntiScalar>>) -> Self {
        let trait_def = Arc::new(RawTraitDefinition {
            documentation: "Sandbox".to_string(),
            names: TraitNames::just("Sandbox"),
            owner: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            arity: TraitArity::Zero,
            output: Arc::new(RwLock::new(TraitTypeConsensus::NoVotes)),
            op: Arc::new(Default::default()),
            dependencies: Arc::new(Default::default()),
        });
        TraitImplBuilder {
            ga,
            mvs,
            registry: TraitImplRegistry::new(),
            trait_def,
            inline_dependencies: true,
            is_deep_inlining: true,
            specialized: false,
            cycle_detector: vec![],
            multivector_dependencies: Default::default(),
            traits10_dependencies: Default::default(),
            traits11_dependencies: Default::default(),
            traits12i_dependencies: Default::default(),
            traits12f_dependencies: Default::default(),
            traits21_dependencies: Default::default(),
            traits22_dependencies: Default::default(),
            wanted_multi_vecs: Default::default(),
            variables: Arc::new(Mutex::new(HashMap::new())),
            lines: Mutex::new(vec![]),
            return_comment: None,
            return_expr: None,
            return_type: HasNotReturned,
        }
    }

    pub fn mark_as_specialized_implementation(&mut self) {
        self.specialized = true;
    }

    pub fn comment<C: Into<String>>(&mut self, comment: C) {
        self.lines.lock().push(CommentOrVariableDeclaration::Comment(Cow::Owned(comment.into())))
    }

    pub fn variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&self, var_name: V, expr: Expr) -> Variable<ExprType> {
        self.comment_variable_impl(None::<String>, var_name, expr.expression_type(), expr.into_any_expression())
    }

    pub fn comment_variable<C: Into<String>, V: Into<String>, ExprType, Expr: Expression<ExprType>>(&mut self, comment: C, var_name: V, expr: Expr) -> Variable<ExprType> {
        self.comment_variable_impl(Some(comment), var_name, expr.expression_type(), expr.into_any_expression())
    }

    fn comment_variable_impl<C: Into<String>, V: Into<String>, ExprType>(&self, comment: Option<C>, var_name: V, expr_type: ExprType, expr: AnyExpression) -> Variable<ExprType> {
        match &expr {
            AnyExpression::Int(IntExpr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Float(FloatExpr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => return Variable { expr_type, decl: v.decl.clone() },
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Variable(v), .. }) => return Variable { expr_type, decl: v.decl.clone() },
            _ => {}
        }

        let var_name = var_name.into();
        let unique_name = make_var_name_unique(&self.variables, var_name);
        let decl = Arc::new(RawVariableDeclaration {
            comment: comment.map(|it| Cow::Owned(it.into())),
            name: unique_name.clone(),
            expr: Some(Arc::new(RwLock::new(expr))),
            force_inline: Arc::new(AtomicBool::new(false)),
        });
        let mut vars = self.variables.lock();
        let existing = vars.insert(unique_name.clone(), Arc::downgrade(&decl));
        assert!(existing.is_none(), "Variable {unique_name:?} is already taken");
        self.lines.lock().push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&decl)));
        Variable { expr_type, decl }
    }

    fn coerce_variable<V: Into<String>, ExprType, Expr: Expression<ExprType>>(&self, name_if_new_var: V, expr: Expr) -> Variable<ExprType> {
        match expr.try_into_variable() {
            Some(already_done) => already_done,
            None => self.variable(name_if_new_var, expr),
        }
    }

    pub fn return_expr<ExprType, Expr: Expression<ExprType>>(self, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        self.comment_return_impl(None::<String>, expr)
    }

    pub fn comment_return<C: Into<String>, ExprType, Expr: Expression<ExprType>>(self, comment: C, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        self.comment_return_impl(Some(comment), expr)
    }

    pub fn construct(&self, dynamic_multi_vector: DynamicMultiVector) -> Option<MultiVectorExpr> {
        dynamic_multi_vector.construct(&self)
    }

    pub fn construct_exact(&self, dynamic_multi_vector: DynamicMultiVector) -> Option<MultiVectorExpr> {
        dynamic_multi_vector.construct_exact(&self)
    }

    pub(crate) fn note_wanted(&self, sig: BTreeSet<BasisSignature>) {
        self.wanted_multi_vecs.lock().insert(sig);
    }

    fn comment_return_impl<C: Into<String>, ExprType, Expr: Expression<ExprType>>(self, comment: Option<C>, expr: Expr) -> Option<TraitImplBuilder<AntiScalar, ExprType>> {
        let return_type = expr.expression_type();
        Some(TraitImplBuilder {
            ga: self.ga.clone(),
            mvs: self.mvs.clone(),
            registry: self.registry,
            // trait_def: self.trait_def,
            trait_def: self.trait_def,
            inline_dependencies: false,
            cycle_detector: self.cycle_detector,
            multivector_dependencies: self.multivector_dependencies,
            traits10_dependencies: self.traits10_dependencies,
            traits11_dependencies: self.traits11_dependencies,
            traits12i_dependencies: self.traits12i_dependencies,
            traits12f_dependencies: self.traits12f_dependencies,
            traits21_dependencies: self.traits21_dependencies,
            traits22_dependencies: self.traits22_dependencies,
            wanted_multi_vecs: self.wanted_multi_vecs,
            variables: self.variables,
            lines: self.lines,
            return_comment: comment.map(|it| it.into()),
            return_expr: Some(expr.into_any_expression()),
            return_type,
            specialized: self.specialized,
            is_deep_inlining: self.is_deep_inlining,
        })
    }

    fn inline_by_copy_existing_10<T: TraitDef_1_Type_0_Args + ?Sized>(&self, trait_key: &TraitKey, raw_impl: Arc<RawTraitImplementation>) -> Option<Variable<T::Output>> {
        let mut var_replacements: Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)> = vec![];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_11<T: TraitDef_1_Type_1_Arg + ?Sized, Expr: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self = param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_21<T: TraitDef_2_Types_1_Arg + ?Sized, Expr: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let old_self = param_self();
        let mut var_replacements = vec![(old_self, new_self)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_22<T: TraitDef_2_Types_2_Args + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<MultiVector>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_12i<T: TraitDef_1_Type_2_Args_i32 + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<Integer>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_by_copy_existing_12f<T: TraitDef_1_Type_2_Args_f32 + ?Sized, Expr1: Expression<MultiVector>, Expr2: Expression<Float>>(
        &self,
        trait_key: &TraitKey,
        raw_impl: Arc<RawTraitImplementation>,
        owner: Expr1,
        other: Expr2,
    ) -> Option<Variable<T::Output>> {
        let new_self = self.coerce_variable("self", owner).decl.clone();
        let new_other = self.coerce_variable("other", other).decl.clone();
        let old_self = param_self();
        let old_other = param_other();
        let mut var_replacements = vec![(old_self, new_self), (old_other, new_other)];
        self.inline_the_lines(&mut var_replacements, &raw_impl.lines);
        let mut return_expr = raw_impl.return_expr.clone();
        for (old, new) in var_replacements.iter() {
            // Update all variables used in this expression
            return_expr.substitute_variable(old.clone(), new.clone());
        }
        let return_expr_type = T::Output::of_expr(&return_expr)?;
        let var = self.comment_variable_impl(raw_impl.return_comment.clone(), trait_key.as_lower_snake(), return_expr_type, return_expr);
        Some(var)
    }

    fn inline_the_lines(&self, var_replacements: &mut Vec<(Arc<RawVariableDeclaration>, Arc<RawVariableDeclaration>)>, lines: &Vec<CommentOrVariableDeclaration>) {
        let mut our_lines = self.lines.lock();
        let their_lines = lines;
        for line in their_lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => our_lines.push(CommentOrVariableDeclaration::Comment(c.clone())),
                CommentOrVariableDeclaration::VarDec(old_decl) => {
                    let Some(old_decl) = old_decl.upgrade() else { continue };
                    let new_var_comment = old_decl.comment.clone();
                    let new_var_name = make_var_name_unique(&self.variables, old_decl.name.0.clone());
                    let mut new_var_expr = old_decl.expr.as_ref().expect("Non-Parameter Variables are always initialized").read().clone();
                    for (old, new) in var_replacements.iter() {
                        // Update all variables used in this expression
                        new_var_expr.substitute_variable(old.clone(), new.clone());
                    }
                    // And create the new replacement declaration for this variable
                    let new_decl = Arc::new(RawVariableDeclaration {
                        comment: new_var_comment,
                        name: new_var_name,
                        expr: Some(Arc::new(RwLock::new(new_var_expr))),
                        force_inline: Arc::new(AtomicBool::new(false)),
                    });
                    // Then add it to the list
                    var_replacements.push((old_decl.clone(), new_decl.clone()));
                    // Then add it to lines
                    our_lines.push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&new_decl)))
                }
            }
        }
    }
}

impl<const AntiScalar: BasisElement, ExprType> TraitImplBuilder<AntiScalar, ExprType> {

    fn into_trait10(self, owner: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeParam, vec![])
    }

    fn into_trait11(self, owner: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeAndDataParam, vec![])
    }

    fn into_trait21(self, owner: MultiVector, other: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeAndDataParam, vec![(ExpressionType::Class(other), Param::TypeParam)])
    }

    fn into_trait22(self, owner: MultiVector, other: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeAndDataParam, vec![(ExpressionType::Class(other), Param::TypeAndDataParam)])
    }

    fn into_trait12i(self, owner: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeAndDataParam, vec![(ExpressionType::Int(Integer), Param::DataParam)])
    }

    fn into_trait12f(self, owner: MultiVector) -> Option<Arc<RawTraitImplementation>> {
        self.into_trait_xx(owner, Param::TypeAndDataParam ,vec![(ExpressionType::Float(Float), Param::DataParam)])
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn into_trait_xx(
        self,
        owner: MultiVector,
        owner_param: Param,
        other_params: Vec<TraitParam>,
    ) -> Option<Arc<RawTraitImplementation>> {

        let lookup = TraitOperationsLookup {
            traits10: &self.traits10_dependencies,
            traits11: &self.traits11_dependencies,
            traits12i: &self.traits12i_dependencies,
            traits12f: &self.traits12f_dependencies,
            traits21: &self.traits21_dependencies,
            traits22: &self.traits22_dependencies,
        };

        // This shouldn't be a problem because of type level state and function visibilities
        let mut return_expr = self.return_expr.expect("Must have return expression in order to register");
        let mut lines = self.lines.into_inner();
        let return_type = return_expr.expression_type();

        let trait_key = self.trait_def.names.trait_key;
        let we_are_debugging = Arc::new(AtomicBool::new(false));
        let copy_pasta_preamble = once_cell::unsync::Lazy::<String, _>::new(|| {
            we_are_debugging.store(true, Release);
            let mut copy_pasta = format!("// Debuggable Copy-Pasta: impl {}", trait_key.final_name);
            let mut open_brace = false;
            for (ty, param) in other_params.iter() {
                if param.is_type_param() {
                    if !open_brace {
                        copy_pasta.push('<');
                        open_brace = true;
                    }
                    copy_pasta.push_str(match ty {
                        ExpressionType::Int(_) => "i32",
                        ExpressionType::Float(_) => "f32",
                        ExpressionType::Vec2(_) => "Simd32x2",
                        ExpressionType::Vec3(_) => "Simd32x3",
                        ExpressionType::Vec4(_) => "Simd32x4",
                        ExpressionType::Class(m) => m.name(),
                    });
                }
            }
            if open_brace { copy_pasta.push('>') }
            copy_pasta.push_str(" for ");
            copy_pasta.push_str(owner.name());
            copy_pasta.push('\n');
            if owner_param.is_data_param() {
                copy_pasta.push_str("let slf = multivec_var(\"self\", &");
                copy_pasta.push_str(owner.name());
                copy_pasta.push_str(", None);\n");
            }
            let mut qty_other = 0;
            for (other_ty, other_param) in other_params.iter() {
                if !other_param.is_data_param() {
                    continue;
                }
                let suffix = if qty_other > 0 {
                    format!("_{}", qty_other + 1)
                } else {
                    String::new()
                };
                qty_other += 1;
                copy_pasta.push_str("let other");
                copy_pasta.push_str(&suffix);
                copy_pasta.push_str(" = ");
                copy_pasta.push_str(match other_ty {
                    ExpressionType::Int(_) => "int",
                    ExpressionType::Float(_) => "float",
                    ExpressionType::Vec2(_) => "vec2",
                    ExpressionType::Vec3(_) => "vec3",
                    ExpressionType::Vec4(_) => "vec4",
                    ExpressionType::Class(_) => "multivec",
                });
                copy_pasta.push_str("_var(\"other");
                copy_pasta.push_str(&suffix);
                copy_pasta.push_str("\"");
                if let ExpressionType::Class(m) = &other_ty {
                    copy_pasta.push_str(", &");
                    copy_pasta.push_str(m.name());
                }
                copy_pasta.push_str(", None);\n");
            }
            copy_pasta
        });
        macro_rules! create_copy_pasta {
            ($lines:ident) => {{
                let mut copy_pasta: String = copy_pasta_preamble.clone();
                for line in $lines.iter() {
                    copy_pasta = format!("{copy_pasta}{line:?}\n");
                }
                copy_pasta.push_str("let mut the_return: ");
                copy_pasta.push_str(match return_type {
                    ExpressionType::Int(_) => "IntExpr",
                    ExpressionType::Float(_) => "FloatExpr",
                    ExpressionType::Vec2(_) => "Vec2Expr",
                    ExpressionType::Vec3(_) => "Vec3Expr",
                    ExpressionType::Vec4(_) => "Vec4Expr",
                    ExpressionType::Class(_) => "MultiVectorExpr",
                });
                copy_pasta.push_str(" = ");
                copy_pasta = format!("{copy_pasta}{:?}", DebugExpression::new(true, &return_expr));
                copy_pasta.push_str(";\n");
                copy_pasta
            }};
        }
        macro_rules! do_copy_pasta {
            ($n:literal) => { do_copy_pasta!(lines, $n) };
            ($lines:ident, $n:literal) => {
                tracing::event!(Level::DEBUG, $n);
                tracing::event!(Level::DEBUG, debuggable_copy_pasta = create_copy_pasta!($lines));
            };
        }

        do_copy_pasta!("Debuggable Copy-Pasta (initial contents):");

        'outer: loop {
            let span = tracing::span!(Level::DEBUG, "inlining_variables");
            let span_entered = span.enter();
            'inner: loop {
                // Scan through the lines in reverse, drop unused variables

                // TODO I think it should be possible to add a flag to the TraitImplBuilder
                //  that indicates whether or not extra-aggressive simplification should be used
                //  and then we read that flag here and maybe don't slice_to_float unless necessary.
                //  We can then use git diffs to see which traits need it on, and trigger the flag
                //  in those implementations. Why though? Because simplification is expensive
                //  enough as it is, it is annoying to undo and redo transposition over and over.

                return_expr.slice_to_floats();
                return_expr.transposing_simplify();

                let mut i = lines.len();
                while i > 0 {
                    i -= 1;
                    let CommentOrVariableDeclaration::VarDec(vd) = &lines[i] else { continue };
                    match vd.upgrade() {
                        None => drop(lines.remove(i)),
                        Some(vd) => {
                            if let Some(v) = &vd.expr {
                                let mut expr = v.write();
                                expr.slice_to_floats();
                                expr.transposing_simplify();
                            }
                        }
                    }
                }

                let mut needs_more_inlining = false;
                for line in lines.iter() {
                    if line.needs_more_inlining() {
                        needs_more_inlining = true;
                        // Normally variable declarations will automatically inline during
                        // simplification if the Arc only has one strong holder.
                        // However, by performing float slicing prior to simplification,
                        // the Arc can/will be cloned, and simplification won't have collapsed it
                        // again by the time it reaches the point it can/should inline a single
                        // use variable. Therefore, we set the force_inline flag.
                        if let CommentOrVariableDeclaration::VarDec(var) = &line {
                            if let Some(var) = var.upgrade() {
                                var.force_inline.store(true, Release);
                                // let n = &var.name;
                                // println!("\nneeds more inlining:");
                                // println!("let {n:?}\n");
                            }
                        }
                    }
                }

                if needs_more_inlining {
                    continue 'inner
                } else {
                    break 'inner
                }
            }
            drop(span_entered);
            do_copy_pasta!("Debuggable Copy-Pasta (after inlining_variables):");

            // Destructuring simplification of variables that are not used in whole
            let mut dv = DestructurableVariables::new();
            return_expr.scan_for_destructurable_variables(&mut dv);
            let mut debug_lines = vec![];
            let mut should_destructure = dv.needs_destructuring();
            let mut did_destructure = false;
            let mut i = 0;

            let span = tracing::span!(Level::DEBUG, "destructuring_variables");
            let span_entered = span.enter();
            'inner: while i < lines.len() {
                // Foo i=2 j=0
                // Bar i=1 j=1
                // Baz i=0 j=2
                let j = lines.len() - i - 1;

                let CommentOrVariableDeclaration::VarDec(vd) = &lines[j] else { continue };
                let vd = vd.upgrade().expect("unused variables were eliminated");
                if !should_destructure.contains(&vd.clone().into()) {
                    if let Some(e) = &vd.expr {
                        let expr = e.read();
                        expr.scan_for_destructurable_variables(&mut dv);
                        should_destructure = dv.needs_destructuring();
                    }
                    i += 1;
                    if we_are_debugging.load(Acquire) {
                        debug_lines.push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&vd)));
                    }
                    continue 'inner
                }
                if let Some(vd) = &vd.expr {
                    tracing::debug!("destructure (before):\n{:?}", DebugExpression::new(true, &*vd.read()));
                }
                let new_vars = Self::destructure_variable_if_applicable(self.variables.clone(), vd.clone());
                if let Some(vd) = &vd.expr {
                    tracing::debug!("destructure (after):\n{:?}", DebugExpression::new(true, &*vd.read()));
                }

                if new_vars.is_empty() && !vd.force_inline.load(Acquire) {
                    if let Some(e) = &vd.expr {
                        let expr = e.read();
                        expr.scan_for_destructurable_variables(&mut dv);
                        should_destructure = dv.needs_destructuring();
                    }
                    i += 1;
                    if we_are_debugging.load(Acquire) {
                        debug_lines.push(CommentOrVariableDeclaration::VarDec(Arc::downgrade(&vd)));
                    }
                    continue 'inner
                }
                // TODO this branch is probably not necessary
                if let Some(expr) = &vd.expr {
                    let mut expr = expr.write();
                    expr.slice_to_floats();
                }

                did_destructure = true;

                // Foo i=2 j=0
                // Bar i=1 j=1
                let removed_line = lines.remove(j);
                if we_are_debugging.load(Acquire) {
                    debug_lines.push(removed_line);
                } else {
                    drop(removed_line);
                }

                let l = new_vars.len();
                for new_var in new_vars {

                    if let Some(e) = &new_var.expr {
                        let expr = e.read();
                        expr.scan_for_destructurable_variables(&mut dv);
                        should_destructure = dv.needs_destructuring();
                        if expr.is_memory_read_and_not_compute() {
                            new_var.force_inline.store(true, Release);
                        }
                    }

                    let new_line = CommentOrVariableDeclaration::VarDec(Arc::downgrade(&new_var));
                    if we_are_debugging.load(Acquire) {
                        debug_lines.push(new_line.clone());
                    }

                    // Foo i=2 j=0
                    // Bar i=1 j=1
                    // Baz_x i=0 j=2
                    // Baz_y i=0 j=3
                    // Baz_z i=0 j=4
                    lines.insert(lines.len() - i, new_line);
                }

                // Foo i=4 j=0
                // Bar i=3 j=1
                // Baz_x i=2 j=2
                // Baz_y i=1 j=3
                // Baz_z i=0 j=4
                i += l;
            }
            drop(span_entered);

            if did_destructure {
                debug_lines.reverse();
                do_copy_pasta!(debug_lines, "Debuggable Copy-Pasta (after destructuring_variables):");
                drop(debug_lines);
                continue 'outer
            } else {
                break 'outer;
            }
        }

        // TODO so.... now we've done a lot of fancy simplification...
        //  but dare we go even further? First distribute down... then factor out? Ugh.
        //  See impl UnitizedRadiusNormSquared for MultiVector
        //  Or  impl AntiConstraintViolation for Line
        //  The reason this is difficult/annoying is because simplification requires
        //  distributing things out, so that terms can cancel. But this means performing
        //  factorization to reduce the number of operations will go against the grain
        //  of simplification.
        // TODO then after pulling out major factors, it might be possible to convert
        //  lots of product into a "machine level dot product" of the whole groups
        //  instead of individually multiplying and adding floats. This would also be
        //  difficult and complicated.


        if self.trait_def.names.trait_key.final_name != "Zero" {
            // We do not implement traits that are statically guaranteed to return zero.
            // We would rather say the trait is not implemented.
            // Otherwise, you could implement almost any trait for almost any combination of classes.

            // Sometimes expressions won't simplify all the way to zero until this very
            // last step where we have inlined single use variables. So here we are.

            // A few exceptions are...
            // - The "Zero" trait
            // - Traits like "Grade" or "AntiGrade" that return raw numerals instead of multivectors

            if let AnyExpression::Class(v) = &return_expr {
                if v.is_zero() {
                    return None;
                }
            }
        }

        let mut statistics = VectoredOperationsTracker::zero();
        statistics += return_expr.count_operations(&lookup);
        for line in lines.iter() {
            if let CommentOrVariableDeclaration::VarDec(vd) = line {
                let Some(vd) = vd.upgrade() else { continue };
                let Some(v) = &vd.expr else { continue };
                statistics += v.read().count_operations(&lookup);
            }
        }

        let ti = Arc::new(RawTraitImplementation {
            definition: self.trait_def,
            owner: (ExpressionType::Class(owner.clone()), owner_param),
            other_params,
            lines,
            return_comment: self.return_comment,
            return_expr,
            statistics,
        });
        let w = self.wanted_multi_vecs.into_inner();
        for mv in w {
            self.mvs.note_wanted(mv, ti.clone());
        }
        Some(ti)
    }

    fn destructure_variable_if_applicable(
        variables: Arc<Mutex<HashMap<(String, usize), Weak<RawVariableDeclaration>>>>,
        rvd: Arc<RawVariableDeclaration>,
    ) -> Vec<Arc<RawVariableDeclaration>> {
        let base_name = &rvd.name.0;
        let Some(vd) = &rvd.expr else { return vec![] };

        let make_a_var= |expr: AnyExpression, suffix: &str| {
            Arc::new(RawVariableDeclaration {
                comment: None,
                name: make_var_name_unique(&variables, format!("{base_name}_{suffix}")),
                expr: Some(Arc::new(RwLock::new(expr))),
                force_inline: Arc::new(AtomicBool::new(false)),
            })
        };
        let make_a_var_2 = |expr: AnyExpression, suffix: String| {
            make_a_var(expr, suffix.as_str())
        };

        // TODO there are some cases where a variable gets used as a whole because of swizzling,
        //  but if you inspect the variable it actually has a lot of zeroes in it.
        //  impl AntiProjectViaHorizonOnto<MultiVector> for DualNum
        //  let anti_wedge_g1 = Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0);
        //  later we see several float access, but also lots of swizzling on only x, y, and z


        // TODO maybe we should allow destructuring Products if they are just one term with coefficients
        //  impl AntiConstraintViolation for AntiMotor
        let mut ae = vd.write();
        match &mut *ae {
            AnyExpression::Vec2(Vec2Expr::Gather1(xy)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(xy.take_as_owned()), "x");
                *xy = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                vec![x_decl]
            }
            AnyExpression::Vec2(Vec2Expr::Gather2(x, y)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(x.take_as_owned()), "x");
                *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                let y_decl = make_a_var(AnyExpression::Float(y.take_as_owned()), "y");
                *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                vec![x_decl, y_decl]
            }
            AnyExpression::Vec2(v2) if matches!(v2, Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => {
                rvd.force_inline.store(true, Release);
                *v2 = Vec2Expr::Gather2(
                    FloatExpr::access_vec_2(v2.clone(), 0),
                    FloatExpr::access_vec_2(v2.take_as_owned(), 1),
                );
                vec![]
            }
            /*
            // AnyExpression::Vec2(v2) if matches!(v2, Vec2Expr::Truncate3to2(box Vec3Expr::Variable(..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v2 = Vec2Expr::Gather2(
            //         FloatExpr::access_vec_2(v2.clone(), 0),
            //         FloatExpr::access_vec_2(v2.take_as_owned(), 1),
            //     );
            //     vec![]
            // }
            // AnyExpression::Vec2(v2) if matches!(v2, Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box Vec3Expr::Variable(..), ..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v2 = Vec2Expr::Gather2(
            //         FloatExpr::access_vec_2(v2.clone(), 0),
            //         FloatExpr::access_vec_2(v2.take_as_owned(), 1),
            //     );
            //     vec![]
            // }
            // AnyExpression::Vec2(v2) if matches!(v2, Vec2Expr::Truncate4to2(box Vec4Expr::Variable(..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v2 = Vec2Expr::Gather2(
            //         FloatExpr::access_vec_2(v2.clone(), 0),
            //         FloatExpr::access_vec_2(v2.take_as_owned(), 1),
            //     );
            //     vec![]
            // }
            // AnyExpression::Vec2(v2) if matches!(v2, Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v2 = Vec2Expr::Gather2(
            //         FloatExpr::access_vec_2(v2.clone(), 0),
            //         FloatExpr::access_vec_2(v2.take_as_owned(), 1),
            //     );
            //     vec![]
            // }
            */
            AnyExpression::Vec2(Vec2Expr::Product(v, last_factor)) if v.len() == 1 => match &mut v[0] {
                (Vec2Expr::Gather1(xy), exponent) if last_factor[0] == last_factor[1] => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(xy.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *xy = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    vec![x_decl]
                }
                (Vec2Expr::Gather2(x, y), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(x.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    let y_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(y.take_as_owned(), *exponent)], last_factor[1])), "y");
                    *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                    vec![x_decl, y_decl]
                }
                (v2, exponent) if matches!(v2, Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _))  => {
                    rvd.force_inline.store(true, Release);
                    *ae = AnyExpression::Vec2(Vec2Expr::Gather2(
                        FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.clone(), 0), *exponent)], last_factor[0]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.take_as_owned(), 1), *exponent)], last_factor[1]),
                    ));
                    vec![]
                }
                /*
                // (v2, exponent) if matches!(v2, Vec2Expr::Truncate3to2(box Vec3Expr::Variable(..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v2 = Vec2Expr::Gather2(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.take_as_owned(), 1), *exponent)], last_factor[1]),
                //     );
                //     vec![]
                // }
                // (v2, exponent) if matches!(v2, Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box Vec3Expr::Variable(..), ..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v2 = Vec2Expr::Gather2(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.take_as_owned(), 1), *exponent)], last_factor[1]),
                //     );
                //     vec![]
                // }
                // (v2, exponent) if matches!(v2, Vec2Expr::Truncate4to2(box Vec4Expr::Variable(..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v2 = Vec2Expr::Gather2(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.take_as_owned(), 1), *exponent)], last_factor[1]),
                //     );
                //     vec![]
                // }
                // (v2, exponent) if matches!(v2, Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v2 = Vec2Expr::Gather2(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_2(v2.take_as_owned(), 1), *exponent)], last_factor[1]),
                //     );
                //     vec![]
                // }
                 */
                _ => vec![],
            }
            AnyExpression::Vec3(Vec3Expr::Gather1(xyz)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(xyz.take_as_owned()), "x");
                *xyz = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                vec![x_decl]
            }
            AnyExpression::Vec3(Vec3Expr::Gather3(x, y, z)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(x.take_as_owned()), "x");
                *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                let y_decl = make_a_var(AnyExpression::Float(y.take_as_owned()), "y");
                *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                let z_decl = make_a_var(AnyExpression::Float(z.take_as_owned()), "z");
                *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                vec![x_decl, y_decl, z_decl]
            }
            AnyExpression::Vec3(Vec3Expr::Extend2to3(xy, z)) => {
                rvd.force_inline.store(true, Release);
                let xy_decl = make_a_var(AnyExpression::Vec2(xy.take_as_owned()), "xy");
                *xy = Vec2Expr::Variable(RawVariableInvocation { decl: xy_decl.clone(), });
                let z_decl = make_a_var(AnyExpression::Float(z.take_as_owned()), "z");
                *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                vec![xy_decl, z_decl]
            }
            AnyExpression::Vec3(v3) if matches!(v3, Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => {
                rvd.force_inline.store(true, Release);
                *v3 = Vec3Expr::Gather3(
                    FloatExpr::access_vec_3(v3.clone(), 0),
                    FloatExpr::access_vec_3(v3.clone(), 1),
                    FloatExpr::access_vec_3(v3.take_as_owned(), 2),
                );
                vec![]
            }
            /*
            // AnyExpression::Vec3(v3) if matches!(v3, Vec3Expr::Truncate4to3(box Vec4Expr::Variable(..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v3 = Vec3Expr::Gather3(
            //         FloatExpr::access_vec_3(v3.clone(), 0),
            //         FloatExpr::access_vec_3(v3.clone(), 1),
            //         FloatExpr::access_vec_3(v3.take_as_owned(), 2),
            //     );
            //     vec![]
            // }
            // AnyExpression::Vec3(v3) if matches!(v3, Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..))) => {
            //     rvd.force_inline.store(true, Release);
            //     *v3 = Vec3Expr::Gather3(
            //         FloatExpr::access_vec_3(v3.clone(), 0),
            //         FloatExpr::access_vec_3(v3.clone(), 1),
            //         FloatExpr::access_vec_3(v3.take_as_owned(), 2),
            //     );
            //     vec![]
            // }
            */
            AnyExpression::Vec3(Vec3Expr::Product(v, last_factor)) if v.len() == 1 => match &mut v[0] {
                (Vec3Expr::Gather1(xyz), exponent) if last_factor[0] == last_factor[1] && last_factor[0] == last_factor[2] => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(xyz.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *xyz = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    vec![x_decl]
                }
                (Vec3Expr::Gather3(x, y, z), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(x.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    let y_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(y.take_as_owned(), *exponent)], last_factor[1])), "y");
                    *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                    let z_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(z.take_as_owned(), *exponent)], last_factor[2])), "z");
                    *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                    vec![x_decl, y_decl, z_decl]
                }
                (Vec3Expr::Extend2to3(xy, z), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let xy_decl = make_a_var(AnyExpression::Vec2(Vec2Expr::Product(vec![(xy.take_as_owned(), *exponent)], [last_factor[0], last_factor[1]])), "xy");
                    *xy = Vec2Expr::Variable(RawVariableInvocation { decl: xy_decl.clone(), });
                    let z_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(z.take_as_owned(), *exponent)], last_factor[2])), "z");
                    *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                    vec![xy_decl, z_decl]
                }
                (v3, exponent) if matches!(v3, Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => {
                    rvd.force_inline.store(true, Release);
                    *ae = AnyExpression::Vec3(Vec3Expr::Gather3(
                        FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 0), *exponent)], last_factor[0]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 1), *exponent)], last_factor[1]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.take_as_owned(), 2), *exponent)], last_factor[2]),
                    ));
                    vec![]
                }
                /*
                // (v3, exponent) if matches!(v3, Vec3Expr::Truncate4to3(box Vec4Expr::Variable(..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v3 = Vec3Expr::Gather3(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 1), *exponent)], last_factor[1]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.take_as_owned(), 2), *exponent)], last_factor[2]),
                //     );
                //     vec![]
                // }
                // (v3, exponent) if matches!(v3, Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box Vec4Expr::Variable(..), ..))) => {
                //     rvd.force_inline.store(true, Release);
                //     *v3 = Vec3Expr::Gather3(
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 0), *exponent)], last_factor[0]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.clone(), 1), *exponent)], last_factor[1]),
                //         FloatExpr::Product(vec![(FloatExpr::access_vec_3(v3.take_as_owned(), 2), *exponent)], last_factor[2]),
                //     );
                //     vec![]
                // }
                 */
                _ => vec![],
            }
            AnyExpression::Vec4(Vec4Expr::Gather1(xyzw)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(xyzw.take_as_owned()), "x");
                *xyzw = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                vec![x_decl]
            }
            AnyExpression::Vec4(Vec4Expr::Gather4(x, y, z, w)) => {
                rvd.force_inline.store(true, Release);
                let x_decl = make_a_var(AnyExpression::Float(x.take_as_owned()), "x");
                *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                let y_decl = make_a_var(AnyExpression::Float(y.take_as_owned()), "y");
                *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                let z_decl = make_a_var(AnyExpression::Float(z.take_as_owned()), "z");
                *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                let w_decl = make_a_var(AnyExpression::Float(w.take_as_owned()), "w");
                *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                vec![x_decl, y_decl, z_decl, w_decl]
            }
            AnyExpression::Vec4(Vec4Expr::Extend2to4(xy, z, w)) => {
                rvd.force_inline.store(true, Release);
                let xy_decl = make_a_var(AnyExpression::Vec2(xy.take_as_owned()), "xy");
                *xy = Vec2Expr::Variable(RawVariableInvocation { decl: xy_decl.clone(), });
                let z_decl = make_a_var(AnyExpression::Float(z.take_as_owned()), "z");
                *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                let w_decl = make_a_var(AnyExpression::Float(w.take_as_owned()), "w");
                *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                vec![xy_decl, z_decl, w_decl]
            }
            AnyExpression::Vec4(Vec4Expr::Extend3to4(xyz, w)) => {
                rvd.force_inline.store(true, Release);
                let xyz_decl = make_a_var(AnyExpression::Vec3(xyz.take_as_owned()), "xyz");
                *xyz = Vec3Expr::Variable(RawVariableInvocation { decl: xyz_decl.clone(), });
                let w_decl = make_a_var(AnyExpression::Float(w.take_as_owned()), "w");
                *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                vec![xyz_decl, w_decl]
            }
            AnyExpression::Vec4(v4) if matches!(v4, Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => {
                rvd.force_inline.store(true, Release);
                *v4 = Vec4Expr::Gather4(
                    FloatExpr::access_vec_4(v4.clone(), 0),
                    FloatExpr::access_vec_4(v4.clone(), 1),
                    FloatExpr::access_vec_4(v4.clone(), 2),
                    FloatExpr::access_vec_4(v4.take_as_owned(), 3),
                );
                vec![]
            }
            AnyExpression::Vec4(Vec4Expr::Product(v, last_factor)) if v.len() == 1 => match &mut v[0] {
                (Vec4Expr::Gather1(xyzw), exponent) if last_factor[0] == last_factor[1] && last_factor[0] == last_factor[2] && last_factor[0] == last_factor[3] => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(xyzw.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *xyzw = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    vec![x_decl]
                }
                (Vec4Expr::Gather4(x, y, z, w), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let x_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(x.take_as_owned(), *exponent)], last_factor[0])), "x");
                    *x = FloatExpr::Variable(RawVariableInvocation { decl: x_decl.clone(), });
                    let y_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(y.take_as_owned(), *exponent)], last_factor[1])), "y");
                    *y = FloatExpr::Variable(RawVariableInvocation { decl: y_decl.clone(), });
                    let z_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(z.take_as_owned(), *exponent)], last_factor[2])), "z");
                    *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                    let w_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(w.take_as_owned(), *exponent)], last_factor[3])), "w");
                    *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                    vec![x_decl, y_decl, z_decl, w_decl]
                }
                (Vec4Expr::Extend2to4(xy, z, w), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let xy_decl = make_a_var(AnyExpression::Vec2(Vec2Expr::Product(vec![(xy.take_as_owned(), *exponent)], [last_factor[0], last_factor[1]])), "xy");
                    *xy = Vec2Expr::Variable(RawVariableInvocation { decl: xy_decl.clone(), });
                    let z_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(z.take_as_owned(), *exponent)], last_factor[2])), "z");
                    *z = FloatExpr::Variable(RawVariableInvocation { decl: z_decl.clone(), });
                    let w_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(w.take_as_owned(), *exponent)], last_factor[3])), "w");
                    *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                    vec![xy_decl, z_decl, w_decl]
                }
                (Vec4Expr::Extend3to4(xyz, w), exponent) => {
                    rvd.force_inline.store(true, Release);
                    let xyz_decl = make_a_var(AnyExpression::Vec3(Vec3Expr::Product(vec![(xyz.take_as_owned(), *exponent)], [last_factor[0], last_factor[1], last_factor[1]])), "xyz");
                    *xyz = Vec3Expr::Variable(RawVariableInvocation { decl: xyz_decl.clone(), });
                    let w_decl = make_a_var(AnyExpression::Float(FloatExpr::Product(vec![(w.take_as_owned(), *exponent)], last_factor[3])), "w");
                    *w = FloatExpr::Variable(RawVariableInvocation { decl: w_decl.clone(), });
                    vec![xyz_decl, w_decl]
                }
                (v4, exponent) if matches!(v4, Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr: box MultiVectorVia::Variable(_), .. }, _)) => {
                    rvd.force_inline.store(true, Release);
                    *ae = AnyExpression::Vec4(Vec4Expr::Gather4(
                        FloatExpr::Product(vec![(FloatExpr::access_vec_4(v4.clone(), 0), *exponent)], last_factor[0]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_4(v4.clone(), 1), *exponent)], last_factor[1]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_4(v4.clone(), 2), *exponent)], last_factor[2]),
                        FloatExpr::Product(vec![(FloatExpr::access_vec_4(v4.take_as_owned(), 3), *exponent)], last_factor[3]),
                    ));
                    vec![]
                }
                _ => vec![],
            }
            AnyExpression::Class(MultiVectorExpr { expr: box MultiVectorVia::Construct(parts), .. }) => {
                if !parts.is_empty() {
                    rvd.force_inline.store(true, Release);
                }
                let mut result = vec![];
                for (i, part) in parts.iter_mut().enumerate() {
                    match part {
                        MultiVectorGroupExpr::JustFloat(f) => {
                            let f_decl = make_a_var_2(AnyExpression::Float(f.take_as_owned()), format!("g{i}"));
                            *f = FloatExpr::Variable(RawVariableInvocation { decl: f_decl.clone(), });
                            result.push(f_decl);
                        }
                        MultiVectorGroupExpr::Vec2(v) => {
                            let v_decl = make_a_var_2(AnyExpression::Vec2(v.take_as_owned()), format!("g{i}"));
                            *v = Vec2Expr::Variable(RawVariableInvocation { decl: v_decl.clone(), });
                            result.push(v_decl);
                        }
                        MultiVectorGroupExpr::Vec3(v) => {
                            let v_decl = make_a_var_2(AnyExpression::Vec3(v.take_as_owned()), format!("g{i}"));
                            *v = Vec3Expr::Variable(RawVariableInvocation { decl: v_decl.clone(), });
                            result.push(v_decl);
                        }
                        MultiVectorGroupExpr::Vec4(v) => {
                            let v_decl = make_a_var_2(AnyExpression::Vec4(v.take_as_owned()), format!("g{i}"));
                            *v = Vec4Expr::Variable(RawVariableInvocation { decl: v_decl.clone(), });
                            result.push(v_decl);
                        }
                    }
                }
                result
            }
            _ => vec![]
        }
    }
}


impl<const AntiScalar: BasisElement, ExprType: TraitResultType> TraitImplBuilder<AntiScalar, ExprType> {
    fn finish_inline<V: Into<String>>(self, b: &mut TraitImplBuilder<AntiScalar, HasNotReturned>, var_name: V) -> Option<Variable<ExprType>> {
        let mut outer_lines = b.lines.lock();
        let inner_lines = self.lines.into_inner();
        for line in inner_lines.into_iter() {
            outer_lines.push(line);
        }

        // Don't deadlock in next method
        drop(outer_lines);

        // If there was no return expression provided, assume the implementation "failed"
        // under normal circumstances like some combination of classes that doesn't produce a result
        let var = b.comment_variable_impl(self.return_comment, var_name, self.return_type, self.return_expr?);



        // Add inner invocations to dependencies (this is a shallow inline, not deep inline, so
        // the trait that we inlined might have further delegated traits that are invoked and
        // not inlined themselves)
        b.traits10_dependencies.extend(self.traits10_dependencies);
        b.traits11_dependencies.extend(self.traits11_dependencies);
        b.traits12i_dependencies.extend(self.traits12i_dependencies);
        b.traits12f_dependencies.extend(self.traits12f_dependencies);
        b.traits21_dependencies.extend(self.traits21_dependencies);
        b.traits22_dependencies.extend(self.traits22_dependencies);

        let inner_dependencies = self.trait_def.dependencies.lock().clone();
        b.trait_def.dependencies.lock().extend(inner_dependencies);

        Some(var)
    }

    fn finish_deep_inline(self) -> Option<<ExprType as TraitResultType>::Expr> {
        let mut r = self.return_expr?;
        r.deep_simplify();
        ExprType::select_expr(r)
    }
}
