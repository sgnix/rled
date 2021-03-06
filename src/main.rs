use std::io;
use std::io::BufRead;

mod ledger;
use ledger::account::Account;
use ledger::title::Title;

#[derive(Debug)]
struct Transaction {
    title: Title,
    accounts: Vec<Account>,
}

struct ErrorMsg<'a> {
    line_no: i32,
    message: &'a str,
}

fn parse<'a>(source: impl BufRead) -> Result<Vec<Transaction>, ErrorMsg<'a>> {
    let mut num = 0i32;
    let mut lines_iter = source.lines();
    let mut txs: Vec<Transaction> = vec![];

    while let Some(line) = lines_iter.next() {
        num += 1;
        if let Ok(line) = line {
            if let Some(title) = Title::parse(&line) {
                let mut accs: Vec<Account> = vec![];
                while let Some(Ok(line)) = lines_iter.next() {
                    num += 1;
                    if line.starts_with(' ') {
                        if let Some(account) = Account::parse(&line) {
                            accs.push(account);
                        } else {
                            return Err(ErrorMsg {
                                line_no: num,
                                message: &"Account format error",
                            });
                        }
                    }
                }
                txs.push(Transaction {
                    title: title,
                    accounts: accs,
                });
            }
        } else {
            return Err(ErrorMsg {
                line_no: num,
                message: &"Input error",
            });
        }
    }

    Err(ErrorMsg {
        line_no: 1,
        message: &"todo",
    })
}

fn main() {
    let result = parse(io::stdin().lock());
}
