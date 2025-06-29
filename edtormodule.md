Preview Document Endpoint (GET /api/editor/preview/{conversion_id}):
Gets the converted Word document preview
Shows how the LaTeX/OCR text looks in Word format
Returns HTML representation of the Word document
Update Preview Endpoint (PUT /api/editor/preview/{conversion_id}):
Allows real-time updates to the document
Updates formatting, text corrections, etc.
Returns updated preview
Preview Sections Endpoint (GET /api/editor/preview/{conversion_id}/sections):
Gets specific sections of the document
Useful for large documents
Enables partial loading for better performance
Preview Metadata Endpoint (GET /api/editor/preview/{conversion_id}/metadata):
Gets document metadata
Page count, sections, formatting info
Document structure