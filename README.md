# banking-transaction
Small transaction program in which data is recorded in csv file using rust language

A CLI for Banking and Transaction management
It have following features:
- you'll pass username, transaction(credit or debit) and the amount to the CLI, if the user already exists, add/remove the amount from their balance. 
- if the user does not exist, create a new user with zero balance.
- store the users and their balance in a csv file.
- store the transactions for a particular user in another file like "<user_name>.csv" , put all user transaction in a seperate folder inside the project as "transaction", so transaction files will look like "transaction/user1.csv"
- format of transactions csv should have the following columns : S.no , transaction type, transaction amount, balance (after transaction) , datetime(when the transaction was made)
