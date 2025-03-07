implementing rga3d;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
public static const ProjectOrthogonallyOntoInfix project_orthogonally_onto = ProjectOrthogonallyOntoInfix();
public struct ProjectOrthogonallyOntoInfix {}
public struct ProjectOrthogonallyOntoInfixPartial<A> { internal A a; }
__include "./impls/project_orthogonally_onto";
