use std::process::Command;
use std::fs;
use std::io::Write;
use uuid::Uuid;
use std::error::Error;

pub struct DocumentConverter;

impl DocumentConverter {
    /// Converts a string of LaTeX into a Word document (.docx) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_docx(latex_content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let docx_path = temp_dir.join(format!("{}.docx", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        // Wrapping it in a basic document structure and a math environment ($...$)
        // helps pandoc correctly interpret pseudo-LaTeX from the model.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\begin{{document}}\n{}\n\\end{{document}}",
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

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&docx_path);

        Ok(docx_bytes)
    }

    /// Converts a string of LaTeX into an ODT document (.odt) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_odt(latex_content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let odt_path = temp_dir.join(format!("{}.odt", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\begin{{document}}\n{}\n\\end{{document}}",
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

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&odt_path);

        Ok(odt_bytes)
    }

    /// Converts a string of LaTeX into a PDF document (.pdf) byte vector.
    /// This function requires `pandoc` to be installed and available in the system's PATH.
    pub fn latex_to_pdf(latex_content: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Generate unique filenames in the system's temp directory to avoid conflicts.
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = std::env::temp_dir();
        let tex_path = temp_dir.join(format!("{}.tex", unique_id));
        let pdf_path = temp_dir.join(format!("{}.pdf", unique_id));

        // 1. Write LaTeX content to a temporary .tex file.
        let mut file = fs::File::create(&tex_path)?;
        let full_latex_doc = format!(
            "\\documentclass{{article}}\n\\begin{{document}}\n{}\n\\end{{document}}",
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

        // 4. Clean up the final temporary file.
        let _ = fs::remove_file(&pdf_path);

        Ok(pdf_bytes)
    }
} 