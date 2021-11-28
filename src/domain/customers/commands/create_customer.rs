/*! Contains the `CreateCustomer Command` type. */

use crate::domain::{
    customers::*, // list them out, models?
    infra::*, // list them out
    Error,
};

/** Input for a `CreateCustomer Command`. */
#[derive(Clone, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

impl CommandArgs for CreateCustomer {
    type Output = Result<(), Error>;
}

async fn execute(
    command: CreateCustomer,
    transaction: ActiveTransaction,  // from infra?
    store: impl CustomerStore,
) -> Result<(), Error> {
    debug!("creating customer `{}`", command.id);

    let customer = { // is this variable a function type??
        if store.get_customer(command.id)?.is_some() {
            err!("customer `{}` already exists", command.id)?
        } else {
            Customer::new(command.id)?
        }
    };

    store.set_customer(transaction.get(), customer)?;

    info!("customer `{}` created", command.id);

    Ok(())
}

impl Resolver { // what is this?, how to access it
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl Command<CreateCustomer> {
        self.command(|resolver, command: CreateCustomer| async move {
            let store = resolver.customer_store();
            let active_transaction = resolver.active_transaction();

            execute(command, active_transaction, store).await
        })
    }
}

// should tests be included here?
#[cfg(test)]
mod tests {
    use crate::domain::customers::model::store::in_memory_store;

    use super::*;

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateCustomer {
            id: CustomerId::new(),
        };

        execute(create.clone(), ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(execute(create, ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
