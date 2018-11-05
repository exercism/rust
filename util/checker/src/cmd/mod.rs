mod check_stubs_compile;

pub fn check_stubs_compile() -> Result<i32, checker::OSInteractionError> {
    check_stubs_compile::check_stubs_compile()
}
