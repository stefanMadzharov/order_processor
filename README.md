# Order Processor

A Rust-based utility that scans `.cdr` files in a given archive folder, parses product sticker information from file names, reads order data from an Excel file, and generates processed order Excel files for production.

---

## Features

- Recursively searches a directory for `.cdr` files  
- Parses file names into structured sticker entries  
- Infers missing product codes based on similarity thresholds  
- Reads order data from a specified Excel `.xlsx` or `.xls` file  
- Supports configurable sheet and column names  
- Uses external `dimensions.txt` for product dimensions (needed only during compilation)
- Outputs processed order Excel files named `orders_dd_mm_yy.xlsx`  
- Logs parsing errors and warnings for diagnostics  

---

## Configuration (`configs.txt` entries)

| Key                              | Required | Type   | Default Value | Description                                                                 |
|----------------------------------|----------|--------|----------------|-----------------------------------------------------------------------------|
| `archive`                        | Yes      | Path   | –              | Path to the directory containing `.cdr` (CorelDRAW) files.                 |
| `order`                          | Yes      | Path   | –              | Path to the `.xlsx` or `.xls` Excel file with order information.           |
| `sheet_name`                     | No       | String | `Sheet1`       | Name of the sheet in the Excel file to read from.                         |
| `order_amount_column_name`       | No       | String | –              | Optional custom column name for the order amount.                         |
| `inferring_levenshtein_distance` | No       | Float  | `0.93`         | Threshold for inferring missing sticker codes based on description match. |
| `error_output_levenshtein_distance` | No    | Float  | `0.7`          | Threshold for showing similar orders during error reporting. Must be lower than the inferring threshold. |

### Example `configs.txt`

```txt
# Path to .cdr files
archive=./archive/

# Path to Excel order file (.xlsx or .xls)
order=./orders/input.xlsx

# Optional: Sheet name (default is 'Sheet1')
sheet_name=Orders

# Optional: Custom name of the column with order amounts
order_amount_column_name=Total Quantity

# Optional thresholds
inferring_levenshtein_distance=0.92
error_output_levenshtein_distance=0.6
```

Note:
- All entries must be on their own lines with `key=value`.  
- The `archive` must be a valid directory.  
- The `order` file must be a valid `.xlsx` or `.xls` file.  
- The `dimensions` file must exist and be a readable text in the form `WxH` file during compilation
- Optional keys will use defaults if omitted.

---

## Usage

1. Place the compiled binary and `configs.txt` in the same folder.  
2. Make sure input order Excel file are present.  
3. Run the binary:

   - On **Linux/macOS**:

     ```bash
     ./order_processor
     ```

   - On **Windows**:

     ```cmd
     .\order_processor.exe
     ```

4. Output will be saved as `orders_dd_mm_yy.xlsx` in the current directory.

---

## Output

- An Excel file `orders_dd_mm_yy.xlsx` with deduplicated, production-ready order data.
- Parsing errors and inference warnings will be printed to the console.
- Levenshtein-based suggestions help identify potential filename or order mismatches.

---

## Troubleshooting

- Missing config file: Ensure `configs.txt` exists in the working directory.  
- Invalid `archive` path: Make sure the path points to a folder.  
- Invalid `order` file: Ensure the file exists and is `.xlsx` or `.xls`.  
- Using defaults: If thresholds or optional fields are not provided, the application warns and uses sensible defaults.

---

## Default Threshold Warnings

The program will print a warning like this if no custom threshold is set:

```txt
!!!WARNING: USING DEFAULT INFERRING LEVENSHTEIN DISTANCE OF 0.93!!!
For custom value set in 'configs.txt', e.g.
inferring_levenshtein_distance=0.9
```

Likewise for `error_output_levenshtein_distance`.
