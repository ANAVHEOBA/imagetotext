Stage 1: PDF Ingestion & Disassembly
This stage takes the raw PDF and breaks it into its fundamental parts.
PDF Loader: The system ingests the uploaded PDF file.
Content Classifier: It inspects the PDF's internal structure.
If it's image-based (a scanned document): It converts each page into a high-resolution image (e.g., 300 DPI PNG). These images are sent down the "Vision Path."
If it's digital: It does not convert the whole page to an image. Instead, it uses a library (like PyMuPDF or pdfplumber) to extract:
All text blocks with their content, font size, font weight (bold/italic), and coordinates (x, y position).
All embedded images as separate files, with their coordinates.
This collection of text and image assets is sent down the "Digital Path."
Stage 2: Content Analysis (The Two Paths)
This is where the actual "understanding" happens.
A. Vision Path (for Scanned Pages)
Each page image is sent to the Qwen VLM.
Crucial Change in Prompting: You don't just ask Qwen to "transcribe the text." You give it a more complex instruction to act as a layout analysis engine.
New Prompt Example: "Analyze the following page image. Identify all structural elements including headings (H1, H2, H3), paragraphs, lists (bulleted or numbered), tables, and mathematical equations. Output the result as a structured JSON object where each element has a 'type' and 'content' field. For tables, provide the data in a nested array. For math, provide the LaTeX."
The output from Qwen for each page is no longer a flat text string, but a structured representation (like JSON) of that single page.

B. Digital Path (for Digital PDFs)
This path minimizes the use of Qwen to save cost and improve accuracy.
Structural Analysis (No OCR): A rule-based parser analyzes the extracted text blocks.
Headings: Identified by large font sizes and bold weights.
Lists: Identified by indentation and leading characters (e.g., •, 1., a)).
Paragraphs: Standard text blocks.
Surgical OCR: The images that were extracted (likely figures, diagrams, and complex equations) are sent individually to Qwen for transcription. This is far more efficient than OCRing the whole page.
The system combines the parsed text structure with the OCR results from the images to produce the same structured JSON format as the Vision Path.
Stage 3: Document-Level Reconstruction
This is the most critical stage for preserving document structure. It takes the structured data from all the individual pages and stitches them together into a single, logical document.
Heading Hierarchy: It builds a document tree based on the heading levels (H1, H2, etc.).
Continuity Merging: It detects when a paragraph or a list item continues from the bottom of one page to the top of the next and merges them.
Global Renumbering: It re-numbers all figures, tables, and equations sequentially throughout the entire document, correcting any page-specific numbering.
Output: The final output of this stage is a single, master "document object" representing the entire PDF's logical structure.

Stage 4: Final Formatting Engine
This stage takes the master document object and translates it into the desired output format.
To LaTeX/HTML/Markdown: Traverse the document object tree and generate the corresponding tags (\section{}, <h1>, #, etc.) for each element.
To DOCX/ODT: Use a library (like python-docx or your existing pandoc flow) to programmatically build the document. You'd add headings, paragraphs, and tables one by one based on the structure in your master object.