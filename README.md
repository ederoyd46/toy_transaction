# Toy Transaction Engine

## Summary

This program takes a csv input and processes each transaction per client, outputting the result to stdout. Errors and additional information is logged to stderr.

## Quick Start

- Clone the repository
- Build and Run the unit tests

```sh
cargo test
```

- Run the application

```sh
cargo run -- etc/transactions_calculations.csv > output.csv
```

## CSV Input Format

| Field  |                     Type                      |            Notes |
| :----- | :-------------------------------------------: | ---------------: |
| type   | deposit/withdrawal/dispute/resolve/chargeback | Transaction Type |
| client |                      u16                      |        Client ID |
| tx     |                      u32                      |   Transaction ID |
| amount |                      f64                      |           Amount |

```csv
type,       client, tx, amount
deposit,    1,      1,  1.0122
withdrawal, 1,      2,  1.5
dispute,    1,      2
resolve,    1,      2
dispute,    1,      2
chargeback, 1,      2
```

## Test data
The etc directory contains test data files;
  - transactions_calculations.csv - Some basic transactions
  - transactions_disputes.csv - For Testing disputes
  - transactions_calculations_large.csv - Larger number of transactions for performance testing
  
## Error Handling

- The project uses csv and serde to parse the files, this guarantees the types are correct for processing.
- Fatal Errors i.e. IO errors are logged to stderr. To be improved..
- Use Result<T,E> and try not to Panic unless the file is missing or can not be parsed.

## Assumptions

- The chronological list of transactions do *not* break the workflow, i.e. a dispute is resolved then chargeback can not happen.
- Logging to log file not required. Messages to stderr is are just for information purposes.

## Future Improvements

- Performance testing, this has only been tested with smaller datasets
  - A larger datafile has been created under ```etc/transactions_calculations_large``` however this is still under 2000 entries.
  - Check memory usage, make sure we're not copying data unnecessarily.
- Improve input validation, serde just gives None if it can't parse the number.
  - _note invalid u16 value cause posOverflow on client. Can this be handled better?_
- Modify the error handling so we match on the error kind rather than just printing out the whole error.
  - This will also help with displaying serde validation messages
- Add Integration tests.
- All data is stored in memory, given a large dataset, it would be better to persist the transactions per client files, then read per client when building the RunningState.
  - Add trait to the processor_state so we can have a data store implementation.
- Encapsulate the structs better so the properties can't get written to directly where not needed. Currently used for asserts on tests.
- Handle duplicate deposit/withdrawal transaction ids? (might not be an issue).
- All code is run on a single thread, the import could be split into multiple threads. 
  - This would depend on the maximum expected number of transactions as it may not be worth the added complexity.
- Refactor TransactionRunningState - not sure I like how we're updating the state.
- Document functions where appropriate.
  