use anyhow::Result;

use crate::models::actor::Actor;
use crate::{actors::game::messages::Message, game::Game};

pub struct GameActor {
    game: Game,
}

impl GameActor {
    pub fn new(client_id: u16) -> Self {
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

impl Actor<Message> for GameActor {
    type Output = ();

    fn handle(&mut self, Message(cx, command): Message) -> Result<Self::Output> {
        Ok(())
    }
}
