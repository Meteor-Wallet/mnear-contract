mod setup;
use setup::*;

#[tokio::test]
async fn test_setup() {
    let worker = near_workspaces::sandbox().await.unwrap();
    let context = Context::new(&worker, None).await;
    check!(view context.lst_contract.get_version());
}

#[tokio::test]
async fn test_upgrade() {
    let worker = near_workspaces::sandbox().await.unwrap();
    let context = Context::new(&worker, None).await;
    assert_eq!(
        context
            .lst_contract
            .acl_grant_role(
                &context.root,
                Role::UpgradableCodeStager.into(),
                context.alice.id()
            )
            .await
            .unwrap()
            .json::<Option<bool>>()
            .unwrap(),
        Some(true)
    );
    check!(view context.lst_contract.get_version());
    check!(context
        .lst_contract
        .up_stage_code(&context.alice, "../../res/lst.wasm"));
    let staged_code_hash = context
        .lst_contract
        .up_staged_code_hash()
        .await
        .unwrap()
        .unwrap();
    check!(context
        .lst_contract
        .up_deploy_code(&context.alice, staged_code_hash.clone()), "Insufficient permissions for method up_deploy_code restricted by access control. Requires one of these roles: [\\\"UpgradableCodeDeployer\\\", \\\"DAO\\\"]");
    check!(context
        .lst_contract
        .up_deploy_code(&context.root, staged_code_hash));
    check!(view context.lst_contract.get_version());
}
