use std::process::Command;
use std::fs;
use std::io::Write;
use uuid::Uuid;
use std::error::Error;

#[derive(Debug)]
pub struct ConversionResult {
    pub content: Vec<u8>,
    pub size: u64,
    pub mime_type: String,
}

pub struct DocumentConverter;

impl DocumentConverter {
    /// Converts a string of LaTeX into a Word document (.docx) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_docx(latex_content: &str) -> Result<ConversionResult, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let docx_path = temp_dir.join(format!("{}.docx", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\usepackage{{amsmath}}\n\\usepackage{{amssymb}}\n\\begin{{document}}\n{}\n\\end{{document}}",
            latex_content
        );
        file.write_all(full_latex_doc.as_bytes())?;

        // 2. Execute the pandoc command-line tool.
        let output = Command::new("pandoc")
            .arg(tex_path.to_str().unwrap())
            .arg("-o")
            .arg(docx_path.to_str().unwrap())
            .output()?;

        // Ensure temporary files are cleaned up regardless of pandoc's success.
        let _ = fs::remove_file(&tex_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Pandoc execution failed: {}", stderr).into());
        }

        // 3. Read the generated .docx file into bytes.
        let docx_bytes = fs::read(&docx_path)?;
        let file_size = docx_bytes.len() as u64;

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&docx_path);

        Ok(ConversionResult {
            content: docx_bytes,
            size: file_size,
            mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        })
    }

    /// Converts a string of LaTeX into an ODT document (.odt) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_odt(latex_content: &str) -> Result<ConversionResult, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let odt_path = temp_dir.join(format!("{}.odt", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\usepackage{{amsmath}}\n\\usepackage{{amssymb}}\n\\begin{{document}}\n{}\n\\end{{document}}",
            latex_content
        );
        file.write_all(full_latex_doc.as_bytes())?;

        // 2. Execute the pandoc command-line tool.
        let output = Command::new("pandoc")
            .arg(tex_path.to_str().unwrap())
            .arg("-o")
            .arg(odt_path.to_str().unwrap())
            .output()?;

        // Ensure temporary files are cleaned up regardless of pandoc's success.
        let _ = fs::remove_file(&tex_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Pandoc execution failed: {}", stderr).into());
        }

        // 3. Read the generated .odt file into bytes.
        let odt_bytes = fs::read(&odt_path)?;
        let file_size = odt_bytes.len() as u64;

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&odt_path);

        Ok(ConversionResult {
            content: odt_bytes,
            size: file_size,
            mime_type: "application/vnd.oasis.opendocument.text".to_string(),
        })
    }

    /// Converts a string of LaTeX into a PDF document (.pdf) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_pdf(latex_content: &str) -> Result<ConversionResult, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let pdf_path = temp_dir.join(format!("{}.pdf", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\usepackage{{amsmath}}\n\\usepackage{{amssymb}}\n\\begin{{document}}\n{}\n\\end{{document}}",
            latex_content
        );
        file.write_all(full_latex_doc.as_bytes())?;

        // 2. Execute the pandoc command-line tool with PDF output.
        let output = Command::new("pandoc")
            .arg(tex_path.to_str().unwrap())
            .arg("-o")
            .arg(pdf_path.to_str().unwrap())
            .arg("--pdf-engine=xelatex")  // Use xelatex for better Unicode support
            .output()?;

        // Ensure temporary files are cleaned up regardless of pandoc's success.
        let _ = fs::remove_file(&tex_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Pandoc execution failed: {}", stderr).into());
        }

        // 3. Read the generated .pdf file into bytes.
        let pdf_bytes = fs::read(&pdf_path)?;
        let file_size = pdf_bytes.len() as u64;

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&pdf_path);

        Ok(ConversionResult {
            content: pdf_bytes,
            size: file_size,
            mime_type: "application/pdf".to_string(),
        })
    }

    /// Converts a string of LaTeX into HTML for preview purposes.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    /// Returns both the HTML content and any embedded CSS for styling.
    pub fn latex_to_html_preview(latex_content: &str) -> Result<ConversionResult, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let html_path = temp_dir.join(format!("{}.html", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\usepackage{{amsmath}}\n\\usepackage{{amssymb}}\n\\begin{{document}}\n{}\n\\end{{document}}",
            latex_content
        );
        file.write_all(full_latex_doc.as_bytes())?;

        // 2. Execute pandoc with HTML5 output and MathJax for math rendering
        let output = Command::new("pandoc")
            .arg(tex_path.to_str().unwrap())
            .arg("-o")
            .arg(html_path.to_str().unwrap())
            .arg("-s")  // Standalone HTML with header
            .arg("--mathjax")  // Use MathJax for equations (better browser support than MathML)
            .arg("-t")
            .arg("html5")  // HTML5 output
            .arg("--highlight-style=tango")  // Add syntax highlighting
            .arg("--variable=colorlinks:true")  // Color links for better visibility
            .output()?;

        // Clean up the tex file
        let _ = fs::remove_file(&tex_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Pandoc error: {}", stderr);  // Log the error for debugging
            return Err(format!("Pandoc execution failed: {}", stderr).into());
        }

        // 3. Read the generated HTML file
        let html_content = match fs::read_to_string(&html_path) {
            Ok(content) => content.into_bytes(),
            Err(e) => {
                eprintln!("Failed to read HTML file: {}", e);  // Log the error for debugging
                return Err(e.into());
            }
        };

        let file_size = html_content.len() as u64;

        // 4. Clean up the HTML file
        let _ = fs::remove_file(&html_path);

        Ok(ConversionResult {
            content: html_content,
            size: file_size,
            mime_type: "text/html".to_string(),
        })
    }
} 