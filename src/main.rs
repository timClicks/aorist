use lib::concept::AoristConcept;
use lib::utils::get_data_setup;

fn main() -> Result<(), String> {
    //let _foo = attributes::KeyAttribute1{};
    let mut setup = get_data_setup();
    setup.compute_uuids();
    setup.compute_constraints();
    setup.traverse_constrainable_children(Vec::new());
    /*for dataset in setup.get_datasets().unwrap() {
        println!("{}", dataset.to_yaml());
        println!("{}", dataset.get_presto_schemas());
    }
    for user in setup.get_users().unwrap() {
        println!("{}", user.to_yaml());
    }
    for group in setup.get_groups().unwrap() {
        println!("{}", group.to_yaml());
    }
    for role_binding in setup.get_role_bindings().unwrap() {
        println!("{}", role_binding.to_yaml());
    }*/
    /*for (_k, v) in setup.get_pipelines()? {
        println!("{}", v);
    }*/
    for constraint in setup.get_constraints() {
        println!(
            "Constraint: {} on {}({})",
            constraint.read().unwrap(),
            "ParsedDataSetup",
            setup.get_uuid()
        );
        for downstream_rw in constraint.read().unwrap().get_downstream_constraints() {
            let downstream = downstream_rw.read().unwrap();
            println!(
                "Requires: {} on {}({})",
                downstream,
                downstream.get_root(),
                downstream.get_root_uuid()
            );
        }
    }
    //perms = setup.get_user_permissions();
    /*
    println!("{}", setup.get_curl_calls(
        "admin".to_string(),
        "eagerLamprey".to_string(),
        "localhost".to_string(),
        1000
    ));
    */
    Ok(())
}
