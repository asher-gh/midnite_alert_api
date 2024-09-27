use std::collections::HashSet;

use crate::model::{Transaction, TxnType};

#[repr(u32)]
#[derive(Eq, PartialEq, Hash)]
pub enum AlertCodes {
    WithdrawalAmountOver100 = 1100,
    ThreeConsecutiveWithdrawals = 30,
    ThreeConsecutiveIncreasingDeposits = 300,
    TotalDepositsOver200 = 123,
}

/*
* get_alerts take a list of transactions (sorted by time in descending order) and returns a set of
* alert codes.
*/
pub async fn get_alerts(
    transactions: &[Transaction],
) -> HashSet<AlertCodes> {
    let mut alerts = HashSet::new();

    if !transactions.is_empty() {
        if let TxnType::Withdrawal = &transactions[0].r#type {
            if transactions[0].amount as u32 > 100 {
                alerts.insert(AlertCodes::WithdrawalAmountOver100);
            }
        }

        if transactions.len() >= 3 {
            let mut consecutive_withdrawals = true;
            for txn in transactions.iter().take(3) {
                if let TxnType::Deposit = txn.r#type {
                    consecutive_withdrawals = false;
                    break;
                }
            }

            if consecutive_withdrawals {
                alerts
                    .insert(AlertCodes::ThreeConsecutiveWithdrawals);
            }

            let mut consecutive_deposits = true;
            for txn in transactions.iter().take(3) {
                if let TxnType::Withdrawal = txn.r#type {
                    consecutive_deposits = false;
                    break;
                }
            }
            if consecutive_deposits
                && transactions[1].amount > transactions[0].amount
                && transactions[2].amount > transactions[1].amount
            {
                alerts.insert(
                    AlertCodes::ThreeConsecutiveIncreasingDeposits,
                );
            }
        }
    }

    if transactions.iter().map(|x| x.amount).sum::<f64>() > 200.0 {
        alerts.insert(AlertCodes::TotalDepositsOver200);
    }

    alerts
}
