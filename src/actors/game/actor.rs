use super::messages::GameCommand;
use crate::entities::actor::AsyncActor;
use crate::{actors::game::messages::Message, entities::game::Game};
use anyhow::Result;
use async_trait::async_trait;

pub struct GameActor {
    game: Game,
}

impl GameActor {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    // fn withdraw(&mut self, tx_id: u32, amount: f32) -> Result<()> {
    //     self.account.withdraw(amount)?;
    //     self.transactions.insert(
    //         tx_id,
    //         TransactionData {
    //             ty: TransactionType::Withdrawal,
    //             amount,
    //             disputed: false,
    //         },
    //     );
    //     Ok(())
    // }

    // fn deposit(&mut self, tx_id: u32, amount: f32) -> Result<()> {
    //     self.account.deposit(amount)?;
    //     self.transactions.insert(
    //         tx_id,
    //         TransactionData {
    //             ty: TransactionType::Deposit,
    //             amount,
    //             disputed: false,
    //         },
    //     );
    //     Ok(())
    // }

    // fn dispute(&mut self, tx_id: u32) -> Result<()> {
    //     let client_id = self.account.get_client();
    //     let tx = self
    //         .transactions
    //         .get_mut(&tx_id)
    //         .ok_or(AccountError::TxNotFound(tx_id, client_id))?;
    //     if tx.ty == TransactionType::Deposit && !tx.disputed {
    //         self.account.held(tx.amount)?;
    //         tx.disputed = true;
    //     }
    //     Ok(())
    // }

    // fn resolve(&mut self, tx_id: u32) -> Result<()> {
    //     let client_id = self.account.get_client();
    //     let tx = self
    //         .transactions
    //         .get_mut(&tx_id)
    //         .ok_or(AccountError::TxNotFound(tx_id, client_id))?;
    //     if tx.disputed {
    //         self.account.free(tx.amount)?;
    //         tx.disputed = false;
    //     }
    //     Ok(())
    // }

    // fn chargeback(&mut self, tx_id: u32) -> Result<()> {
    //     let client_id = self.account.get_client();
    //     let tx = self
    //         .transactions
    //         .get_mut(&tx_id)
    //         .ok_or(AccountError::TxNotFound(tx_id, client_id))?;
    //     if tx.disputed {
    //         self.account.chargeback(tx.amount)?;
    //         tx.disputed = false;
    //     }
    //     Ok(())
    // }
}

#[async_trait]
impl AsyncActor<Message> for GameActor {
    type Output = ();

    async fn handle(&mut self, Message(cx, command): Message) -> Result<Self::Output> {
        let result = match command {
            GameCommand::Ask(to, card) => {}
            GameCommand::End => {}
            GameCommand::Join => {}
            GameCommand::Start => todo!(),
            GameCommand::Status => todo!(),
            GameCommand::MyStatus => todo!(),
        };
        // Check errors ocurred
        Ok(())
    }
}
