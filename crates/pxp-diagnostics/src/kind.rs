use std::fmt::Display;

use pxp_token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    UnexpectedToken { token: Token },
    ExpectedToken { expected: Vec<TokenKind>, found: Token },
    InvalidSpreadOperator,
    InvalidTargetForAttributes,
    CannotMixKeyedAndUnkeyedListEntries,
    AbstractMethodInNonAbstractClass,
    CannotHaveMultipleDefaultArmsInMatch,
    MissingType,
    StandaloneTypeUsedInNullableType,
    StandaloneTypeUsedInUnionType,
    StandaloneTypeUsedInIntersectionType,
    NestedDisjunctiveNormalFormType,
    InvalidBackedEnumType,
    UnitEnumsCannotHaveCaseValues,
    BackedEnumCaseMustHaveValue,
    CannotUseReservedKeywordAsTypeName,
    CannotUseReservedKeywordAsLabel,
    CannotUseReservedKeywordAsConstantName,
    InvalidClassModifier,
    InvalidMethodModifier,
    InvalidPropertyModifier,
    InvalidConstantModifier,
    CannotUseFinalWithAbstract,
    CannotUseFinalWithPrivateOnConstant,
    DuplicateModifier,
    MultipleVisibilityModifiers,
    CannotMixBracketedAndUnbracketedNamespaceDeclarations,
    NestedNamespace,
    PromotedPropertyCannotBeVariadic,
    ForbiddenTypeUsedInProperty,
    ReadonlyPropertyMustHaveType,
    CannotUsePositionalArgumentAfterNamedArgument,
    PositionalArgumentsOnly,
    OnlyAllowedOneArgument,
    ArgumentRequired,
    StaticPropertyCannotBeReadonly,
    ReadonlyPropertyCannotHaveDefaultValue,
    TryMustHaveCatchOrFinally,
    DynamicVariableNotAllowed,
    UnexpectedEndOfFile,
    UnexpectedEndOfFileExpected { expected: Vec<TokenKind> },
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticKind::UnexpectedToken { token } => write!(f, "unexpected token: {:?}", token),
            DiagnosticKind::ExpectedToken { expected, found } => if expected.len() == 1 {
                write!(f, "unexpected token {:?}, expected {}", found, expected.first().unwrap())
            } else {
                write!(f, "unexpected token {:?}, expected one of {}", found, expected.iter().map(|kind| format!("{}", kind)).collect::<Vec<_>>().join(", "))
            },
            DiagnosticKind::InvalidSpreadOperator => write!(f, "cannot use spread operator here"),
            DiagnosticKind::InvalidTargetForAttributes => write!(f, "invalid target for attributes"),
            DiagnosticKind::CannotMixKeyedAndUnkeyedListEntries => write!(f, "cannot mix keyed and unkeyed list entries"),
            DiagnosticKind::AbstractMethodInNonAbstractClass => write!(f, "cannot declare abstract method in non-abstract class"),
            DiagnosticKind::CannotHaveMultipleDefaultArmsInMatch => write!(f, "cannot have multiple default arms in match"),
            DiagnosticKind::MissingType => write!(f, "missing type"),
            DiagnosticKind::StandaloneTypeUsedInNullableType => write!(f, "cannot use standalone type in nullable type"),
            DiagnosticKind::StandaloneTypeUsedInUnionType => write!(f, "cannot use standalone type in union type"),
            DiagnosticKind::StandaloneTypeUsedInIntersectionType => write!(f, "cannot use standalone type in intersection type"),
            DiagnosticKind::NestedDisjunctiveNormalFormType => write!(f, "DNF types cannot be nested"),
            DiagnosticKind::InvalidBackedEnumType => write!(f, "invalid backed enum type, must be `string` or `int`"),
            DiagnosticKind::UnitEnumsCannotHaveCaseValues => write!(f, "unit enums cannot have case values"),
            DiagnosticKind::BackedEnumCaseMustHaveValue => write!(f, "backed enum case must have value"),
            DiagnosticKind::CannotUseReservedKeywordAsTypeName => write!(f, "cannot use reserved keyword as type name"),
            DiagnosticKind::CannotUseReservedKeywordAsLabel => write!(f, "cannot use reserved keyword as label"),
            DiagnosticKind::CannotUseReservedKeywordAsConstantName => write!(f, "cannot use reserved keyword as constant name"),
            DiagnosticKind::InvalidClassModifier => write!(f, "invalid class modifier"),
            DiagnosticKind::InvalidMethodModifier => write!(f, "invalid method modifier"),
            DiagnosticKind::InvalidPropertyModifier => write!(f, "invalid property modifier"),
            DiagnosticKind::InvalidConstantModifier => write!(f, "invalid constant modifier"),
            DiagnosticKind::CannotUseFinalWithAbstract => write!(f, "cannot use final and abstract together"),
            DiagnosticKind::CannotUseFinalWithPrivateOnConstant => write!(f, "private constant cannot be final as it is not visible to other classes"),
            DiagnosticKind::DuplicateModifier => write!(f, "duplicate modifier"),
            DiagnosticKind::MultipleVisibilityModifiers => write!(f, "cannot have multiple visibility modifiers"),
            DiagnosticKind::CannotMixBracketedAndUnbracketedNamespaceDeclarations => write!(f, "cannot mix bracketed and unbracketed namespace declarations"),
            DiagnosticKind::NestedNamespace => write!(f, "cannot nest namespaces"),
            DiagnosticKind::PromotedPropertyCannotBeVariadic => write!(f, "promoted property cannot be variadic"),
            DiagnosticKind::ForbiddenTypeUsedInProperty => write!(f, "forbidden type used in property"),
            DiagnosticKind::ReadonlyPropertyMustHaveType => write!(f, "readonly property must have type"),
            DiagnosticKind::CannotUsePositionalArgumentAfterNamedArgument => write!(f, "cannot use positional argument after named argument"),
            DiagnosticKind::PositionalArgumentsOnly => write!(f, "only positional arguments are allowed"),
            DiagnosticKind::OnlyAllowedOneArgument => write!(f, "only one argument is allowed"),
            DiagnosticKind::ArgumentRequired => write!(f, "argument required"),
            DiagnosticKind::StaticPropertyCannotBeReadonly => write!(f, "static property cannot be readonly"),
            DiagnosticKind::ReadonlyPropertyCannotHaveDefaultValue => write!(f, "readonly property cannot have default value"),
            DiagnosticKind::TryMustHaveCatchOrFinally => write!(f, "try must have catch or finally"),
            DiagnosticKind::DynamicVariableNotAllowed => write!(f, "dynamic variable not allowed"),
            DiagnosticKind::UnexpectedEndOfFile => write!(f, "unexpected end of file"),
            DiagnosticKind::UnexpectedEndOfFileExpected { expected } => if expected.len() == 1 {
                write!(f, "unexpected end of file, expected {}", expected.first().unwrap())
            } else {
                write!(f, "unexpected end of file, expected one of {}", expected.iter().map(|kind| format!("{}", kind)).collect::<Vec<_>>().join(", "))
            },
        }
    }
}