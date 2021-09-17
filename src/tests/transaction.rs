#[test]
fn transaction_without_commit() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        Some("value".to_string())
    );
    Ok(())
}

#[test]
fn transaction_with_commit() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    transaction.commit(std::path::Path::new("./data"))?;

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        Some("value".to_string())
    );
    transaction.commit(std::path::Path::new("./data"))?;

    Ok(())
}

#[test]
fn transaction_update() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    transaction.commit(std::path::Path::new("./data"))?;

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Update((
            "key".to_string(),
            "value_updated".to_string()
        )))?,
        None
    );
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        Some("value_updated".to_string())
    );
    transaction.commit(std::path::Path::new("./data"))?;

    Ok(())
}

#[test]
fn transaction_remove() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    transaction.commit(std::path::Path::new("./data"))?;

    let mut transaction = crate::Transaction::new(&mut table);
    assert_eq!(
        transaction.exec(crate::Request::Remove("key".to_string()))?,
        None
    );
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        None
    );
    transaction.commit(std::path::Path::new("./data"))?;

    Ok(())
}

#[test]
fn transaction_many() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&mut table);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        transaction.commit(std::path::Path::new("./data"))?;
    }

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&mut table);
        assert_eq!(transaction.exec(crate::Request::Find(key))?, Some(value));
        transaction.commit(std::path::Path::new("./data"))?;
    }

    Ok(())
}

#[test]
fn transaction_persist() -> Result<(), Box<dyn std::error::Error>> {
    crate::io::remove_dir(std::path::Path::new("./data"))?;
    let root_node = crate::RootNode::<String, String, 10>::new();
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };

    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&mut table);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        transaction.commit(std::path::Path::new("./data"))?;
    }
    crate::dump(&table.primary, std::path::Path::new("./data"))?;

    for i in 100..200 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&mut table);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        transaction.commit(std::path::Path::new("./data"))?;
    }

    let root_node = crate::load::<String, String, 10>(std::path::Path::new("./data"))?;
    let mut table = crate::Table {
        primary: root_node,
        secondaries: vec![],
    };
    for i in 0..200 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&mut table);
        assert_eq!(transaction.exec(crate::Request::Find(key))?, Some(value));
        transaction.commit(std::path::Path::new("./data"))?;
    }

    Ok(())
}
