use final_project::store::Store;
use final_project::task::{Priority, Status};

#[test]
fn full_add_list_done_remove_flow() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db_path = dir.path().join("test.db");
    let store = Store::open(&db_path)?;

    // add two tasks
    let first = store.add_task("Buy milk", Priority::High)?;
    let second = store.add_task("Read book", Priority::Low)?;
    assert_eq!(first.title, "Buy milk");
    assert_eq!(first.status, Status::Todo);
    assert_eq!(first.priority, Priority::High);
    assert_eq!(second.title, "Read book");
    assert_eq!(second.priority, Priority::Low);

    // list shows both
    let all = store.list_tasks(None, None)?;
    assert_eq!(all.len(), 2);

    // filtering works
    let high_only = store.list_tasks(None, Some(Priority::High))?;
    assert_eq!(high_only.len(), 1);
    assert_eq!(high_only[0].id, first.id);

    // done flips the status
    store.mark_done(first.id)?;
    let done = store.list_tasks(Some(Status::Done), None)?;
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].id, first.id);
    let todo = store.list_tasks(Some(Status::Todo), None)?;
    assert_eq!(todo.len(), 1);
    assert_eq!(todo[0].id, second.id);

    // remove deletes the second task
    store.remove_task(second.id)?;
    let remaining = store.list_tasks(None, None)?;
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, first.id);

    Ok(())
}

#[test]
fn unknown_ids_return_errors() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let store = Store::open(dir.path().join("test.db"))?;

    assert!(store.mark_done(999).is_err());
    assert!(store.remove_task(999).is_err());
    assert!(store.add_task("   ", Priority::Medium).is_err());

    Ok(())
}
