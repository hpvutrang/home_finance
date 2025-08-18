-- Test datas:
-- Some fixed stuffs:
--  * Salary        | Income    --> Asset
--  * Insurance     | Asset     --> Expense
--  * Electricity   | Asset     --> Expense
--  * Sport         | Asset     --> Expense
--  * Rent          | Asset     --> Expense
--  * Services      | Asset     --> Expense
--  * Internet      | Asset     --> Expense
--  * Tax           | Asset     --> Expense

BEGIN; -- Transaction
    INSERT INTO accounts (name, family) 
    SELECT 'Bank', id 
    FROM account_families af
    WHERE af.name = 'Asset';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Insurance', id 
    FROM account_families  af
    WHERE af.name = 'Expense';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Electricity', id 
    FROM account_families af
    WHERE af.name = 'Expense';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Sport', id 
    FROM account_families af
    WHERE af.name = 'Expense';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Rent', id 
    FROM account_families af
    WHERE af.name = 'Expense';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Services', id 
    FROM account_families af
    WHERE af.name = 'Expense';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Internet', id 
    FROM account_families af
    WHERE af.name = 'Services';
    
    INSERT INTO accounts (name, family) 
    SELECT 'Tax', id 
    FROM account_families af
    WHERE af.name = 'Liability';

    INSERT INTO accounts (name, family) 
    SELECT 'Salary', id 
    FROM account_families af
    WHERE af.name = 'Income';
END; -- Transaction

BEGIN;
    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Salary for December', '2024-11-28 11:30:30', 10000, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Salary'
    AND a2.name = 'Bank';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Insurance stuffs', '2024-12-01 10:00:00', 200, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Insurance';
        
    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Electrical bill', '2024-11-30 10:00:00', 79.0, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Electricity';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Fitness park', '2024-11-30 10:00:00', 29.99, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Fitness park';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Renting for December', '2024-11-29 15:00:00', 1000.0, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Rent';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Youtube music', '2024-11-29 15:00:00', 15.0, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Services';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Internet bill for december', '2024-12-02 15:00:00', 30.0, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Internet';

    INSERT INTO entries (description, event_date, amount, credit, debit)
    SELECT 'Prelevement a la source', '2024-12-02 15:00:00', 3000.0, a2.id, a1.id
    FROM accounts a1, accounts a2
    WHERE a1.name = 'Bank'
    AND a2.name = 'Tax';
END;
