#[test]
fn node_insert() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crate::RootNode::<&str, &str, 10>::new();
    crate::Node::insert(&mut index, &"key", "value", false)?;
    assert_eq!(crate::Node::find(&index, &"key"), Some(&"value"));
    Ok(())
}

#[test]
fn node_update() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crate::RootNode::<&str, &str, 10>::new();
    crate::Node::insert(&mut index, &"key", "value", false)?;
    crate::Node::update(&mut index, &"key", "value_updated")?;
    assert_eq!(crate::Node::find(&index, &"key"), Some(&"value_updated"));
    Ok(())
}

#[test]
fn node_remove() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crate::RootNode::<&str, &str, 10>::new();
    crate::Node::insert(&mut index, &"key", "value", false)?;
    crate::Node::remove(&mut index, &"key")?;
    assert_eq!(crate::Node::find(&index, &"key"), None);
    Ok(())
}

#[test]
fn node_insert_many() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crate::RootNode::<String, String, 10>::new();

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        crate::Node::insert(&mut index, &key, value.clone(), false)?;
        assert_eq!(crate::Node::find(&index, &key), Some(&value));
    }

    Ok(())
}

#[test]
fn node_remove_many() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = crate::RootNode::<String, String, 10>::new();

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        crate::Node::insert(&mut index, &key, value, false)?;
    }

    for i in 0..1000 {
        let key = format!("key{}", i);
        crate::Node::remove(&mut index, &key)?;
        assert_eq!(crate::Node::find(&index, &key), None);
    }

    for i in 0..1000 {
        let key = format!("key{}", i);
        assert_eq!(crate::Node::find(&index, &key), None);
    }

    Ok(())
}

#[test]
fn node_dump() -> Result<(), Box<dyn std::error::Error>> {
    crate::io::remove_dir(std::path::Path::new("./data"))?;
    let mut index = crate::RootNode::<String, String, 10>::new();

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        crate::Node::insert(&mut index, &key, value.clone(), false)?;
    }

    crate::dump(&index, std::path::Path::new("./data"))?;
    index = crate::load(std::path::Path::new("./data"))?;

    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        assert_eq!(crate::Node::find(&index, &key), Some(&value));
    }

    Ok(())
}
