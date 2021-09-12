#[test]
fn transaction_without_commit() -> Result<(), Box<dyn std::error::Error>> {
    let root_node = crate::RootNode::new();
    let mut transaction = crate::Transaction::new(&root_node);
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
    let mut root_node = crate::RootNode::new();

    let mut transaction = crate::Transaction::new(&root_node);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    let mut transaction = crate::Transaction::new(&root_node);
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        Some("value".to_string())
    );
    transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    Ok(())
}

#[test]
fn transaction_update() -> Result<(), Box<dyn std::error::Error>> {
    let mut root_node = crate::RootNode::new();

    let mut transaction = crate::Transaction::new(&root_node);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    let mut transaction = crate::Transaction::new(&root_node);
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
    transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    Ok(())
}

#[test]
fn transaction_remove() -> Result<(), Box<dyn std::error::Error>> {
    let mut root_node = crate::RootNode::new();

    let mut transaction = crate::Transaction::new(&root_node);
    assert_eq!(
        transaction.exec(crate::Request::Insert((
            "key".to_string(),
            "value".to_string()
        )))?,
        None
    );
    root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    let mut transaction = crate::Transaction::new(&root_node);
    assert_eq!(
        transaction.exec(crate::Request::Remove("key".to_string()))?,
        None
    );
    assert_eq!(
        transaction.exec(crate::Request::Find("key".to_string()))?,
        None
    );
    transaction.commit(std::path::Path::new("./data"))?(root_node)?;

    Ok(())
}

#[test]
fn transaction_many() -> Result<(), Box<dyn std::error::Error>> {
    let mut root_node = crate::RootNode::new();

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&root_node);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;
    }

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&root_node);
        assert_eq!(transaction.exec(crate::Request::Find(key))?, Some(value));
        root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;
    }

    Ok(())
}

#[test]
fn transaction_persist() -> Result<(), Box<dyn std::error::Error>> {
    crate::io::remove_dir(std::path::Path::new("./data"))?;
    let mut root_node = crate::RootNode::new();

    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&root_node);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;
    }
    crate::dump(&root_node, std::path::Path::new("./data"))?;

    for i in 100..200 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&root_node);
        assert_eq!(
            transaction.exec(crate::Request::Insert((key, value)))?,
            None
        );
        root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;
    }

    root_node = crate::load(std::path::Path::new("./data"))?;
    for i in 0..200 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let mut transaction = crate::Transaction::new(&root_node);
        assert_eq!(transaction.exec(crate::Request::Find(key))?, Some(value));
        root_node = transaction.commit(std::path::Path::new("./data"))?(root_node)?;
    }

    Ok(())
}
