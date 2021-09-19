#[test]
fn simple_secondary_index() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut secondaries: std::collections::HashMap<
        String,
        Box<dyn crate::table::SecondaryIndex<String, String, 10>>,
    > = std::collections::HashMap::new();
    secondaries.insert(
        "value".to_string(),
        Box::new(crate::DefaultSecondaryIndex::new(
            |x| crate::Primitive::String(x),
            |x| {
                if let crate::Primitive::String(x) = x {
                    Some(x)
                } else {
                    None
                }
            },
        )),
    );

    let mut table = crate::Table {
        primary: root_node,
        secondaries,
    };

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Insert((
        "key".to_string(),
        "value".to_string(),
    )))?;

    assert_eq!(
        transaction.select(
            &"value".to_string(),
            &crate::Primitive::String("value".to_string())
        )?,
        {
            let mut keys = std::collections::HashSet::new();
            keys.insert("key".to_string());
            keys
        }
    );

    Ok(())
}

#[test]
fn secondary_index_with_commit() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut secondaries: std::collections::HashMap<
        String,
        Box<dyn crate::table::SecondaryIndex<String, String, 10>>,
    > = std::collections::HashMap::new();
    secondaries.insert(
        "value".to_string(),
        Box::new(crate::DefaultSecondaryIndex::new(
            |x| crate::Primitive::String(x),
            |x| {
                if let crate::Primitive::String(x) = x {
                    Some(x)
                } else {
                    None
                }
            },
        )),
    );

    let mut table = crate::Table {
        primary: root_node,
        secondaries,
    };

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Insert((
        "key".to_string(),
        "value".to_string(),
    )))?;
    transaction.commit(std::path::Path::new("./data"))?;

    let transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.select(
            &"value".to_string(),
            &crate::Primitive::String("value".to_string())
        )?,
        {
            let mut keys = std::collections::HashSet::new();
            keys.insert("key".to_string());
            keys
        }
    );

    Ok(())
}

#[test]
fn secondary_index_update() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut secondaries: std::collections::HashMap<
        String,
        Box<dyn crate::table::SecondaryIndex<String, String, 10>>,
    > = std::collections::HashMap::new();
    secondaries.insert(
        "value".to_string(),
        Box::new(crate::DefaultSecondaryIndex::new(
            |x| crate::Primitive::String(x),
            |x| {
                if let crate::Primitive::String(x) = x {
                    Some(x)
                } else {
                    None
                }
            },
        )),
    );

    let mut table = crate::Table {
        primary: root_node,
        secondaries,
    };

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Insert((
        "key".to_string(),
        "value".to_string(),
    )))?;
    transaction.commit(std::path::Path::new("./data"))?;

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Update((
        "key".to_string(),
        "value2".to_string(),
    )))?;

    assert_eq!(
        transaction.select(
            &"value".to_string(),
            &crate::Primitive::String("value2".to_string())
        )?,
        {
            let mut keys = std::collections::HashSet::new();
            keys.insert("key".to_string());
            keys
        }
    );

    Ok(())
}

#[test]
fn secondary_index_remove() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut secondaries: std::collections::HashMap<
        String,
        Box<dyn crate::table::SecondaryIndex<String, String, 10>>,
    > = std::collections::HashMap::new();
    secondaries.insert(
        "value".to_string(),
        Box::new(crate::DefaultSecondaryIndex::new(
            |x| crate::Primitive::String(x),
            |x| {
                if let crate::Primitive::String(x) = x {
                    Some(x)
                } else {
                    None
                }
            },
        )),
    );

    let mut table = crate::Table {
        primary: root_node,
        secondaries,
    };

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Insert((
        "key".to_string(),
        "value".to_string(),
    )))?;
    transaction.commit(std::path::Path::new("./data"))?;

    let mut transaction = crate::Transaction::new(&mut table);
    transaction.exec(crate::Request::Remove("key".to_string()))?;

    assert_eq!(
        transaction.select(
            &"value".to_string(),
            &crate::Primitive::String("value".to_string())
        )?,
        std::collections::HashSet::new()
    );

    Ok(())
}
