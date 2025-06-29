   .route("/projects", web::post().to(SyncController::create_project))
            .route("/projects", web::get().to(SyncController::list_projects))
            .route("/projects/{id}", web::get().to(SyncController::get_project))
            .route("/projects/{id}/history", web::get().to(SyncController::get_project_history))
            .route("/history", web::get().to(SyncController::get_all_history))











a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/sync/projects \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs" \
  -d '{
    "name": "My First Project",
    "description": "A test project for OCR tasks"
  }' -i
HTTP/1.1 201 Created
content-length: 318
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
content-type: application/json
access-control-expose-headers: x-refresh-token
access-control-allow-credentials: true
date: Sun, 29 Jun 2025 07:11:45 GMT

{"id":"6860e731d5949084ba9c81fb","name":"My First Project","description":"A test project for OCR tasks","cloudinary_folder":"projects/6859ba0689111c681ae642f6/my_first_project","conversion_count":0,"total_storage_bytes":0,"created_at":"2025-06-29 7:11:45.598 +00:00:00","updated_at":"2025-06-29 7:11:45.598 +00:00:00"}a@a:~/imagetotext$ 














a@a:~/imagetotext$ curl -X GET http://localhost:8080/api/sync/projects \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs" -i
HTTP/1.1 200 OK
content-length: 379
content-type: application/json
access-control-allow-credentials: true
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-expose-headers: x-refresh-token
date: Sun, 29 Jun 2025 07:13:03 GMT

{"projects":[{"id":"6860e731d5949084ba9c81fb","name":"My First Project","description":"A test project for OCR tasks","cloudinary_folder":"projects/6859ba0689111c681ae642f6/my_first_project","conversion_count":0,"total_storage_bytes":0,"created_at":"2025-06-29 7:11:45.598 +00:00:00","updated_at":"2025-06-29 7:11:45.598 +00:00:00"}],"total":1,"page":1,"limit":10,"total_pages":1}a@a:~/imagetotext$ 












@a:~/imagetotext$ curl -X GET http://localhost:8080/api/sync/projects/1/5 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs" -i
HTTP/1.1 200 OK
content-length: 378
content-type: application/json
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
access-control-allow-credentials: true
access-control-expose-headers: x-refresh-token
date: Sun, 29 Jun 2025 07:13:57 GMT

{"projects":[{"id":"6860e731d5949084ba9c81fb","name":"My First Project","description":"A test project for OCR tasks","cloudinary_folder":"projects/6859ba0689111c681ae642f6/my_first_project","conversion_count":0,"total_storage_bytes":0,"created_at":"2025-06-29 7:11:45.598 +00:00:00","updated_at":"2025-06-29 7:11:45.598 +00:00:00"}],"total":1,"page":1,"limit":5,"total_pages":1}a@a:~/imagetotext$ 








a@a:~/imagetotext$ curl -X GET "http://localhost:8080/api/sync/projects/6860e731d5949084ba9c81fb/conversions/1/10" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs"
{"conversions":[],"total":0,"page":1,"limit":10,"total_pages":0}a@a:~/imagetotext$ 










a@a:~/imagetotext$ curl -X GET "http://localhost:8080/api/sync/conversions/unassigned/1/10" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs"
{"conversions":[{"job_id":"7c39f32c-1537-4d86-8c57-6228d25755a6","original_filename":"awqq.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_b4db8dac-fca1-442d-988b-9fb9b03fc841","file_size":59723,"status":"completed","created_at":"2025-06-25 0:18:42.983 +00:00:00","completed_at":"2025-06-25 0:18:42.983 +00:00:00"},{"job_id":"d8889a30-0275-48ec-8b78-935eebd1c89d","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_339c8cc0-e930-4cd9-aa22-a9f107ee09c5","file_size":61515,"status":"completed","created_at":"2025-06-24 16:31:39.786 +00:00:00","completed_at":"2025-06-24 16:31:39.786 +00:00:00"},{"job_id":"5366f44c-cc9e-4e46-ab23-e06548e17c72","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_bb550d3a-b5e3-4b39-b3fe-d3fc88c39113","file_size":61515,"status":"completed","created_at":"2025-06-24 11:07:36.983 +00:00:00","completed_at":"2025-06-24 11:07:36.983 +00:00:00"},{"job_id":"ccc3dc5b-335d-49a6-a459-bf81d534fce7","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_b8982b0b-20c8-4416-9b38-1164dee57dd2","file_size":61515,"status":"completed","created_at":"2025-06-24 11:02:15.704 +00:00:00","completed_at":"2025-06-24 11:02:15.704 +00:00:00"},{"job_id":"fc2ab929-b564-45db-a113-7b1c268c0752","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_d8c5427c-7515-46cc-adb1-c3434fe9dbc0","file_size":61515,"status":"completed","created_at":"2025-06-24 10:56:50.544 +00:00:00","completed_at":"2025-06-24 10:56:50.544 +00:00:00"},{"job_id":"2a94d84a-f682-46ad-9db6-e1fa51307bf8","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_0c96301a-cc76-472b-b94c-0700cc009a63","file_size":61515,"status":"completed","created_at":"2025-06-24 10:50:06.783 +00:00:00","completed_at":"2025-06-24 10:50:06.783 +00:00:00"},{"job_id":"4f2c1932-8aa2-4cd7-937c-6b13c6dcdb9b","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_c02efbd3-5e71-40df-aa8c-bc872f4990e9","file_size":61515,"status":"completed","created_at":"2025-06-24 10:37:23.575 +00:00:00","completed_at":"2025-06-24 10:37:23.575 +00:00:00"},{"job_id":"42699abd-46a2-43d1-9a68-e4933ae336d2","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_405a9a07-1b52-40a8-8afd-df510549f3ec","file_size":61515,"status":"completed","created_at":"2025-06-24 9:57:34.684 +00:00:00","completed_at":"2025-06-24 9:57:34.684 +00:00:00"},{"job_id":"f2d55154-0780-4654-a000-710e8b1bf1ec","original_filename":"Screenshot from 2025-06-23 19-08-32.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_34e4e7f1-5f89-47e2-a843-82dd13e2c2e4","file_size":138116,"status":"completed","created_at":"2025-06-24 8:40:26.58 +00:00:00","completed_at":"2025-06-24 8:40:26.58 +00:00:00"},{"job_id":"58797072-f753-45ad-a8ad-a974cab096c6","original_filename":"Screenshot from 2025-06-23 19-08-32.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_5cb72499-5346-4eb3-92e7-19e6bb27b61d","file_size":138116,"status":"completed","created_at":"2025-06-24 0:02:53.444 +00:00:00","completed_at":"2025-06-24 0:02:53.444 +00:00:00"}],"total":11,"page":1,"limit":10,"total_pages":2}a@a:~/imagetotext$ 









a@a:~/imagetotext$ curl -X POST "http://localhost:8080/api/sync/projects/6860e731d5949084ba9c81fb/assign" \
a@a:~/imagetotext$ curl -X POST "http://localhost:8080/api/sync/projects/6860e731d5949084ba9c81fb/assign" \
  -H "Content-Type: application/json" \KV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs" \
  -d '{nversion_ids": [
    "conversion_ids": [86-8c57-6228d25755a6",
      "7c39f32c-1537-4d86-8c57-6228d25755a6",
      "d8889a30-0275-48ec-8b78-935eebd1c89d"
    ]
  }'
{"message":"Conversions assigned successfully","data":null}a@a:~/imagetotext$ 










a@a:~/imagetotext$ curl -X GET "http://localhost:8080/api/sync/projects/6860e731d5949084ba9c81fb/conversions/1/10" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTEyNjc0MzEsImlhdCI6MTc1MTE4MTAzMSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.25IYtv5EQvMgzMVMHU7rHYaymEHgYmyj-JD1It5sHTs"
{"conversions":[{"job_id":"7c39f32c-1537-4d86-8c57-6228d25755a6","original_filename":"awqq.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_b4db8dac-fca1-442d-988b-9fb9b03fc841","file_size":59723,"status":"completed","created_at":"2025-06-25 0:18:42.983 +00:00:00","completed_at":"2025-06-25 0:18:42.983 +00:00:00"},{"job_id":"d8889a30-0275-48ec-8b78-935eebd1c89d","original_filename":"hjjjj.png","cloudinary_url":"https://res.cloudinary.com/dzzvvkwqa/image/upload/ocr_339c8cc0-e930-4cd9-aa22-a9f107ee09c5","file_size":61515,"status":"completed","created_at":"2025-06-24 16:31:39.786 +00:00:00","completed_at":"2025-06-24 16:31:39.786 +00:00:00"}],"total":2,"page":1,"limit":10,"total_pages":1}a@a:~/imagetotext$ 