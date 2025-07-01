API Endpoints
POST /api/conversion/upload - Upload image file
GET /api/conversion/status/{job_id} - Check conversion status
GET /api/conversion/history - Get user's conversion history
DELETE /api/conversion/{job_id} - Delete a conversion


Processing Flow
Upload → Validate file → Store in temp location → Create DB record
Queue → Add to processing queue (Redis/RabbitMQ)
Process → OCR service processes image
Store → Save extracted text to database
Notify → Update status → Return results






a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/conversion/upload   -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM"   -F "file=@Screenshot from 2025-06-23 19-08-32.png"
{"job_id":"f2d55154-0780-4654-a000-710e8b1bf1ec","message":"Image processed and uploaded successfully.","status":"completed"}a@a:~/imagetotext$ 











a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/f2d55154-0780-4654-a000-710e8b1bf1ec \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM"
{"job_id":"f2d55154-0780-4654-a000-710e8b1bf1ec","status":"completed","original_filename":"Screenshot from 2025-06-23 19-08-32.png","extracted_text":"```\nANAVHEOBA/Issend\ngithub.com\n\nMemory usage: 138 MB\n\nCode Issues Pull requests Actions Projects Wiki Security Insights Settings\n\nIssend Public\n\nmain 1 Branch 0 Tags\nGo to file Add file Code\n\nANAVHEOBA op\n89652fa · 2 weeks ago 8 Commits\n\nfolder src op 2 weeks ago\n.gitignore first commit 2 weeks ago\nScreenshot from 2025-04-08 12-12-51.png first commit 2 weeks ago\nadminmodule.md first commit 2 weeks ago\ncryptodata.md push now 2 weeks ago\npackage-lock.json first commit 2 weeks ago\npackage.json push nowsssegj 2 weeks ago\ntransactionflow.md first commit 2 weeks ago\ntsconfig.json push nowsssegj 2 weeks ago\n\nAbout\nNo description, website, or topics provided.\n\nActivity\n0 stars\n0 watching\n0 forks\n\nReleases\nNo releases published\nCreate a new release\n\nPackages\nNo packages published\nPublish your first package\n```","created_at":"2025-06-24 8:40:26.58 +00:00:00","processing_time_ms":32301}a@a:~/imagetotext$ 


















a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/42699abd-46a2-43d1-9a68-e4933ae336d2 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM"
{"job_id":"42699abd-46a2-43d1-9a68-e4933ae336d2","status":"completed","original_filename":"hjjjj.png","extracted_text":"```\nExample 1. Evaluate lim_(x->0, y->0) (x^2 y)/(x^4 + y^2)\n\nSolution.\n(i) lim_(x->0, y->0) (x^2 y)/(x^4 + y^2) = lim_(x->0) (0)/(0 + y^2) = 0 = f_1 (say)\n\n(ii) lim_(x->0, y->0) (x^2 y)/(x^4 + y^2) = lim_(y->0) (0)/(x^4 + 0) = 0 = f_2 (say)\n\nHere, f_1 = f_2, therefore\n\n(iii) Put y = mx\n\nlim_(x->0, y->0) (x^2 mx)/(x^4 + m^2 x^2) = lim_(x->0) (mx)/(x^2 + m^2) = 0 = f_3 (say)\n\nHere, f_1 = f_2 = f_3, therefore\n\n(iv) Put y = mx^2\n\nlim_(x->0, y->0) (x^2 mx^2)/(x^4 + m^2 x^4) = lim_(x->0) (m)/(1 + m^2) = -m/(1 + m^2) = f_4\n\nHere, f_1 = f_2 = f_3 ≠ f_4\n\nThus, limit does not exist.\n\nAns.\n\nExample 2. Evaluate lim_(x->0) (x^3 + y^3)\n\nSolution.\n(i) lim_(x->0) (x^3 + y^3) = lim_(y->0) (0 + y^3) = 0 = f_1 (say)\n\n(ii) lim_(x->0) (x^3 + y^3) = lim_(x->0) (x^3 + 0) = 0 = f_2 (say)\n\nHere, f_1 = f_2, therefore\n\n(iii) Put y = mx\n\nlim_(x->0) (x^3 + y^3) = lim_(x->0) [lim_(y->0) (x^3 + y^3)] = lim_(x->0) (x^3 + m^3 x^3) = 0 = f_3 (say)\n\nHere, f_1 = f_2 = f_3, therefore\n\n(iv) Put y = mx^2\n\nlim_(x->0) (x^3 + y^3) = lim_(x->0) [lim_(y->-oo) (x^3 + y^3)] = lim_(x->0) (x^3 + m^3 x^6)\n\n= lim_(x->0) x^3 (1 + m^3 x^3) = 0 = f_4 (say)\n\nHere, f_1 = f_2 = f_3 = f_4\n\nThus, limit exists with value 0.\n\nAns.\n```","created_at":"2025-06-24 9:57:34.684 +00:00:00","processing_time_ms":34511}a@a:~/imagetotext$ 











Receives LaTeX from OCR.
Cleans and validates the LaTeX.
Converts it to MathML (using latexml, Pandoc, or similar).
Converts MathML to OMML (using Pandoc or mml2omml).
Inserts OMML into a .docx template (using a Rust, Python, or Node.js docx library).
Returns the .docx file to the user.













a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/conversion/upload \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM" \
  -F "file=@hjjjj.png"
{"job_id":"d8889a30-0275-48ec-8b78-935eebd1c89d","message":"Image processed and uploaded successfully.","status":"completed"}a@a:~/imagetotext$ 





a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/d8889a30-0275-48ec-8b78-935eebd1c89d \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM"
{"job_id":"d8889a30-0275-48ec-8b78-935eebd1c89d","status":"completed","original_filename":"hjjjj.png","extracted_text":"Example 1\nEvaluate $\\lim_{y \\to 0} \\frac{x^2 y}{x^2 + y^2}$\n\n1. First, we note that $$\\lim_{y \\to 0} \\frac{x^2 y}{x^2 + y^2} = \\lim_{y \\to 0} \\frac{0}{0 + y^2} = 0 = f_1$$ (say)\n\n2. Therefore...\n\n(iii) Put $y = mx$\n\n$$\\lim_{y \\to 0} \\frac{x^2 mx}{x^2 + m^2 x^2} = \\lim_{x \\to 0} \\frac{mx}{1 + m^2} = 0 = f_3$$ (say)\n\nHere, $f_1 = f_2 = f_3$, therefore\n\n(iv) Put $y = mx^2$\n\n$$\\lim_{y \\to 0} \\frac{x^2 mx^2}{x^2 + m^2 x^4} = \\frac{m}{1 + m^2} = f_4$$\n\nHere, $f_1 = f_2 = f_3 \\neq f_4$\n\nThus, limit does not exist.\n\nAns.\n\nExample 2\nEvaluate $\\lim_{y \\to 0} (x^3 + y^3)$.\n\nSolution.\n\n(i) $\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{y \\to 0} (0 + y^3) = 0 = f_1$ (say)\n\n(ii) $\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{y \\to 0} (x^3 + 0) = 0 = f_2$ (say)\n\nHere, $f_1 = f_2$, therefore\n\n(iii) Put $y = mx$\n\n$\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{x \\to 0} \\left[ \\lim_{y \\to 0} (x^3 + y^3) \\right] = \\lim_{x \\to 0} (x^3 + m^3 x^3) = 0 = f_3$ (say)\n\nHere, $f_1 = f_2 = f_3$, therefore\n\n(iv) Put $y = mx^2$\n\n$\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{x \\to 0} \\left[ \\lim_{y \\to 0} (x^3 + y^3) \\right] = \\lim_{x \\to 0} (x^3 + m^3 x^6)$\n\n$= \\lim_{x \\to 0} x^3 (1 + m^3 x^3) = 0 = f_4$ (say)\n\nHere, $f_1 = f_2 = f_3 = f_4$\n\nThus, limit exists with value 0.\n\nAns.","created_at":"2025-06-24 16:31:39.786 +00:00:00","processing_time_ms":32037}a@a:~/imagetotext$ 











a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/d8889a30-0275-48ec-8b78-935eebd1c89d/download/word   -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4MDc2MzgsImlhdCI6MTc1MDcyMTIzOCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.6u4sAfW9Fr0O4QRJPN9n7q2N8eqmAtzkv824maQt9cM"   -o result.docx
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 10545  100 10545    0     0    329      0  0:00:32  0:00:32 --:--:--  2779














a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/conversion/upload   -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4OTY5MjUsImlhdCI6MTc1MDgxMDUyNSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.FEheMU5T2-H4Cl8YKpLYydmNIpFnesReZrjf1T5OkT4"   -F "file=@awqq.png"
{"job_id":"7c39f32c-1537-4d86-8c57-6228d25755a6","message":"Image processed and uploaded successfully.","status":"completed"}a@a:~/imagetotext$ 





a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/7c39f32c-1537-4d86-8c57-6228d25755a6 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4OTY5MjUsImlhdCI6MTc1MDgxMDUyNSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.FEheMU5T2-H4Cl8YKpLYydmNIpFnesReZrjf1T5OkT4"
{"job_id":"7c39f32c-1537-4d86-8c57-6228d25755a6","status":"completed","original_filename":"awqq.png","extracted_text":"10.1.2 The connection one-form\n\nIn practical computations, we need to separate \\( T_uP \\) into \\( V_uP \\) and \\( H_uP \\) in a systematic way. This can be achieved by introducing a Lie-algebra-valued one-form \\( \\omega \\in \\mathfrak{g} \\otimes T^*P \\) called the connection one-form.\n\nDefinition 10.2. A connection one-form \\( \\omega \\in \\mathfrak{g} \\otimes T^*P \\) is a projection of \\( T_uP \\) onto the vertical component \\( V_uP \\simeq \\mathfrak{g} \\). The projection property is summarized by the following requirements,\n\n(i) \\( \\omega(A^\\#) = A \\quad A \\in \\mathfrak{g} \\)\n\n(ii) \\( R_g^* \\omega = \\operatorname{Ad}_{g^{-1}} \\omega \\)\n\nthat is, for \\( X \\in T_uP \\),\n\n\\( R_g^* \\omega_{\\operatorname{ug}}(X) = \\omega_{\\operatorname{ug}}(R_g X) = g^{-1} \\omega_u(X) g \\).\n\nDefine the horizontal subspace \\( H_uP \\) by the kernel of \\( \\omega \\),\n\n\\( H_uP \\equiv \\{ X \\in T_uP \\mid \\omega(X) = 0 \\} \\).","created_at":"2025-06-25 0:18:42.983 +00:00:00","processing_time_ms":36152}a@a:~/imagetotext$ 










a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/7c39f32c-1537-4d86-8c57-6228d25755a6/download/word \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA4OTY5MjUsImlhdCI6MTc1MDgxMDUyNSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.FEheMU5T2-H4Cl8YKpLYydmNIpFnesReZrjf1T5OkT4" \
  -o result_connection.docx
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 10426  100 10426    0     0   5708      0  0:00:01  0:00:01 --:--:--  5706
a@a:~/imagetotext$ 





















new 

   a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/conversion/upload \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTE0NDUxNjksImlhdCI6MTc1MTM1ODc2OSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.fbopeMpdFev_KRvTYfioZAaIs2zQ5NI-dYxljX0Ml8k" \
  -F "file=@hjjjj.png"
{"job_id":"ff4984f2-c7a6-4af3-a67b-68a6dc2a3450","message":"Image processed and uploaded successfully.","status":"completed"}a@a:~/imagetotext$ 


a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/conversion/ff4984f2-c7a6-4af3-a67b-68a6dc2a3450 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTE0NDUxNjksImlhdCI6MTc1MTM1ODc2OSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.fbopeMpdFev_KRvTYfioZAaIs2zQ5NI-dYxljX0Ml8k"
{"job_id":"ff4984f2-c7a6-4af3-a67b-68a6dc2a3450","status":"completed","original_filename":"hjjjj.png","extracted_text":"Example 1. Evaluate \\(\\lim_{y \\to 0} \\frac{x^2 y}{x^4 + y^2}\\).\n\nSolution.\n(i) \\(\\lim_{y \\to 0} \\frac{x^2 y}{x^4 + y^2} = \\lim_{y \\to 0} \\frac{0}{0 + y^2} = 0 = f_1\\) (say)\n\n(ii) \\(\\lim_{y \\to 0} \\frac{x^2 y}{x^4 + y^2} = \\lim_{x \\to 0} \\frac{0}{x^4 + 0} = 0 = f_2\\) (say)\n\nHere, \\(f_1 = f_2\\), therefore\n\n(iii) Put \\(y = mx\\)\n\n\\(\\lim_{x \\to 0} \\frac{x^2 mx}{x^4 + m^2 x^2} = \\lim_{x \\to 0} \\frac{mx}{x^2 + m^2} = 0 = f_3\\) (say)\n\nHere, \\(f_1 = f_2 = f_3\\), therefore\n\n(iv) Put \\(y = mx^2\\)\n\n\\(\\lim_{x \\to 0} \\frac{x^2 mx^2}{x^4 + m^2 x^4} = \\lim_{x \\to 0} \\frac{m}{1 + m^2} = \\frac{m}{1 + m^2} = f_4\\)\n\nHere, \\(f_1 = f_2 = f_3 \\neq f_4\\)\n\nThus, limit does not exist.\n\nAns.\n\nExample 2. Evaluate \\(\\lim_{y \\to 0} (x^3 + y^3)\\).\n\nSolution.\n(i) \\(\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{y \\to 0} (0 + y^3) = 0 = f_1\\) (say)\n\n(ii) \\(\\lim_{y \\to 0} (x^3 + y^3) = \\lim_{x \\to 0} (x^3 + 0) = 0 = f_2\\) (say)\n\nHere, \\(f_1 = f_2\\), therefore\n\n(iii) Put \\(y = mx\\)\n\n\\(\\lim_{x \\to 0} (x^3 + y^3) = \\lim_{x \\to 0} \\left[ \\lim_{y \\to 0} (x^3 + y^3) \\right] = \\lim_{x \\to 0} (x^3 + m^3 x^3) = 0 = f_3\\) (say)\n\nHere, \\(f_1 = f_2 = f_3\\), therefore\n\n(iv) Put \\(y = mx^2\\)\n\n\\(\\lim_{x \\to 0} (x^3 + y^3) = \\lim_{x \\to 0} \\left[ \\lim_{y \\to 0} (x^3 + y^3) \\right] = \\lim_{x \\to 0} (x^3 + m^3 x^6) = 0 = f_4\\)\n\nHere, \\(f_1 = f_2 = f_3 = f_4\\)\n\nThus, limit exists with value 0.\n\nAns.","created_at":"2025-07-01 8:34:03.389 +00:00:00","processing_time_ms":29412}a@a:~/imagetotext$ 





