use std::assert_eq;

use conquer_struct::{conquer_future, try_conquer_future};

#[derive(Debug, PartialEq, Eq)]
struct Data {
    name: String,
    user_id: usize,
}

#[tokio::test]
async fn test_inner_workings() {
    let data = conquer_future!(Data {
        name: async { "Hello, World!".to_string() },
        user_id: 123,
    })
    .await;

    assert_eq!(
        data,
        Data {
            name: "Hello, World!".to_string(),
            user_id: 123
        }
    )
}

#[tokio::test]
async fn test_inner_workings_try() {
    let data: Result<_, ()> = try_conquer_future!(Data {
        name: async { Ok::<String, ()>("Hello, World!".to_string()) },
        user_id: 123,
    })
    .await;

    assert_eq!(
        data,
        Ok(Data {
            name: "Hello, World!".to_string(),
            user_id: 123
        })
    )
}

#[tokio::test]
async fn test_context_awareness_try() {
    async fn context_aware_function() -> Result<Data, ()> {
        let data = try_conquer_future!(Data {
            name: async { Ok::<String, ()>("Hello, World!".to_string()) },
            user_id: 123,
        })
        .await?;

        Ok(data)
    }

    assert_eq!(
        context_aware_function().await.unwrap(),
        Data {
            name: "Hello, World!".to_string(),
            user_id: 123
        }
    )
}
