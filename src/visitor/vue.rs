use swc_ecma_ast::*;

#[derive(Debug)]
pub struct OptionsComponent {
    // The components object
    pub components: Option<Box<Expr>>,

    // The inject array/expr
    pub inject: Option<Box<Expr>>,

    // The props
    pub props: Option<Box<Expr>>,

    // The data() method
    pub data: Option<Function>,

    // The created() method
    pub created: Option<Function>,

    // The mounted() method
    pub mounted: Option<Function>,

    // The method object
    pub methods: Option<Box<Expr>>,
}
impl Default for OptionsComponent {
    fn default() -> OptionsComponent {
        Self {
            components: None,
            inject: None,
            props: None,
            data: None,
            created: None,
            mounted: None,
            methods: None,
        }
    }
}

#[derive(Debug)]
pub struct CompositionComponent {
    // The components object
    pub components: Option<Box<Expr>>,

    // The props
    pub props: Option<Box<Expr>>,

    // The inject statments
    pub inject_stmts: Option<Vec<Stmt>>,

    // The ref statements, derived from the data method
    pub ref_stmts: Option<Vec<Stmt>>,

    // The statements gathered from the created method
    pub created_stmts: Option<Vec<Stmt>>,

    // The statements to wrapped with onMounted, gathered from the mounted method
    pub mounted_stmts: Option<Vec<Stmt>>,

    // The function declarations
    pub method_decls: Option<Vec<FnDecl>>,
}
impl Default for CompositionComponent {
    fn default() -> CompositionComponent {
        Self {
            components: None,
            props: None,
            inject_stmts: None,
            ref_stmts: None,
            created_stmts: None,
            mounted_stmts: None,
            method_decls: None,
        }
    }
}
