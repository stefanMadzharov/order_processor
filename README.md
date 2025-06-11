# Order Processor

A Rust-based utility that scans `.cdr` files in a given archive folder, parses product sticker information from file names, reads order data from an Excel file, and generates processed order Excel files for production.

---

## Features

- Recursively searches a directory for `.cdr` files  
- Parses file names into structured sticker entries  
- Infers missing product codes when possible  
- Reads input order data from a specified Excel `.xlsx` file  
- Outputs processed order Excel files named `orders_dd_mm_yy.xlsx`  
- Logs parsing errors for archive fixing and warnings

--- 

## Configuration (`configs.txt` entries)
| Key                              | Required | Type   | Default Value | Description                                                                 |
|----------------------------------|----------|--------|----------------|-----------------------------------------------------------------------------|
| `archive`                        | Yes      | Path   | –              | Path to the directory containing `.cdr` (CorelDRAW) files.                 |
| `order`                          | Yes      | Path   | –              | Path to the `.xlsx` Excel file with order information.                     |
| `inferring_levenshtein_distance` | No       | Float  | `0.93`         | Threshold for inferring missing sticker codes based on description match.  |
| `error_output_levenshtein_distance` | No   | Float  | `0.7`          | Threshold for showing similar orders when printing error diagnostics. Should always be lower than `inferring_levenshtein_distance`!      |

Example `configs.txt`
  ```txt
  # Path to directory containing .cdr files (CorelDRAW files)
  archive=path/to/cdr/files

  # Path to the input Excel file (.xlsx) containing order information
  order=path/to/input_order_file.xlsx

  # Optional: Similarity threshold (from 0.0 to 1.0) used when trying to infer missing codes
  # If omitted, defaults to 0.93 (corresponding roughly to 1-2 character edits)
  inferring_levenshtein_distance=0.9

  # Optional: Similarity threshold (from 0.0 to 1.0) used when printing matching orders for errors
  # If omitted, defaults to 0.7
  error_output_levenshtein_distance=0.65

  # Notes:
  # - All values must be on separate lines.
  # - The archive path must be a directory.
  # - The order path must point to a valid .xlsx file.
  # - If optional values are missing, the program will warn and use defaults.

  ```
---

## Usage

1. Place the compiled binary and `configs.txt` in the same folder.  
2. Edit `configs.txt` with the correct paths.  
3. Run the binary
   - On Linux/macOS:

     ```txt 
     ./order_processor
     ```

   - On Windows, double-click the executable or run from Command Prompt so that you can also see the Error Output:

     ```txt 
     .\order_processor.exe
     ```


The program will:

- Scan the archive directory for `.cdr` files  
- Parse and organize stickers from filenames  
- Read order data from the input Excel file  
- Create an output Excel file named `orders_dd_mm_yy.xlsx` (current date)  
- Print any errors or warnings to the console  

---

## Output

- Output Excel file `orders_dd_mm_yy.xlsx` will be saved in the current working directory.  
- Contains grouped and deduplicated sticker order data ready for production.  
- The Error output helps you fix archive problems

---

## Troubleshooting

- Ensure `configs.txt` exists and paths are correct.  
- The `archive` path must be a valid directory.  
- The `order` path must be a valid `.xlsx` file.  
- Check console output for parsing errors or file issues.  
