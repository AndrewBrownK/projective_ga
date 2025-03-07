implementing cga3d;

/// AntiSandwich
/// The anti-sandwich is the dual to the sandwich, using the AntiProduct instead of the Product, and the AntiReverse instead of the Reverse. This is also used to represent geometric transformations, for example reflecting across a plane or rotating around a line. The Sandwich and AntiSandwich are not identical for the purposes of transforming geometry, you simply choose which one to use depending on your geometric interpretation and the algebra. For example, in G(3,0,1) you may interpret grade 1 vectors as points or planes, since they are dual to one another. The sandwich product gives euclidean transformations in the grade 1 = planes interpretation, and the AntiSandwich gives euclidean transformations in the grade 1 = points interpretation.
public static const AntiSandwichInfix anti_sandwich = AntiSandwichInfix();
public struct AntiSandwichInfix {}
public struct AntiSandwichInfixPartial<A> { internal A a; }
__include "./impls/anti_sandwich";
