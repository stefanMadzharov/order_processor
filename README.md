# Order Processor

A Rust-based utility that scans `.cdr` files in a given archive folder, parses product sticker information from file names, reads order data from an Excel file, and generates processed order Excel files for production.

---

## Features

- Recursively searches a directory for `.cdr` files  
- Parses file names into structured sticker entries  
- Infers missing product codes when possible  
- Reads input order data from a specified Excel `.xlsx` file  
- Outputs processed order Excel files named `orders_dd_mm_yy.xlsx`  
- Logs parsing errors and warnings for review  

--- 

## Configuration (`configs.txt` entries)
- On Linux/macOS:
  
  ```txt
  # Path to directory containing .cdr files (CorelDRAW files)
  archive=path/to/cdr/files

  # Path to the input Excel file (.xlsx) containing order information
  order=path/to/input_order_file.xlsx
  ```

- On Windows, double-click the executable or run from Command Prompt:
  
  ```txt 
  archive=C:\\Users\\YourName\\Documents\\archive_with_cdr_files
  order=C:\\Users\\YourName\\Documents\\order_file.xlsx
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

   - On Windows, double-click the executable or run from Command Prompt:

     ```txt 
     order_processor.exe
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

---

## Troubleshooting

- Ensure `configs.txt` exists and paths are correct.  
- The `archive` path must be a valid directory.  
- The `order` path must be a valid `.xlsx` file.  
- Check console output for parsing errors or file issues.  
