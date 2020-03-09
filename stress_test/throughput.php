<?php

$curl = curl_init();

curl_setopt_array($curl, array(
  CURLOPT_PORT => "5000",
  /* CURLOPT_URL => "http://35.228.134.188:5001/render/abandoned.html", */
  CURLOPT_URL => "http://127.0.0.1:5000/render/abandoned.html",
  CURLOPT_RETURNTRANSFER => true,
  CURLOPT_ENCODING => "",
  CURLOPT_MAXREDIRS => 1,
  CURLOPT_TIMEOUT => 30,
  CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
  CURLOPT_CUSTOMREQUEST => "POST",
  CURLOPT_POSTFIELDS => "{\n\t\t\t\"front_link\": \"ponominalu.ru\",\n\t\t\t\"name\": \"Петров Иван Николаевич\",\n\t\t\t\"subevent_full_title\": \"Park-live 2020\",\n\t\t\t\"image_wide_clean\": \"be419aa5c08a0b752a84c3b5725a0d0049cabce1.jpg\",\n\t\t\t\"age\": 18,\n\t\t\t\"date\": \"01.01.2020\",\n\t\t\t\"venue_title\": \"Москонцерт Холл\",\n\t\t\t\"subevent_type\": \"open\",\n\t\t\t\"unsubscribe_link\": \"/unsubscribe\",\n\t\t\t\"cs\": {\n\t\t\t\t\"phone\": \"+77777777777777\",\n\t\t\t\t\"email\": \"johndoe@test.com\",\n\t\t\t\t\"name\": \"johndoe\"\n\t\t\t},\n\t\t\t\"rows\": []\n\t\t}",
  CURLOPT_HTTPHEADER => array(
    "authorization: Token 42e61df3-5c13-49d9-9962-64635a13adf8",
    "content-type: application/json"
  ),
));

function rutime($ru, $rus, $index) {
    return ($ru["ru_$index.tv_sec"]*1000 + intval($ru["ru_$index.tv_usec"]/1000))
     -  ($rus["ru_$index.tv_sec"]*1000 + intval($rus["ru_$index.tv_usec"]/1000));
}

$time_start = microtime(true); 

//dividing with 60 will give the execution time in minutes otherwise seconds

//execution time of the script
$rustart = getrusage();
for ($i = 0; $i < 300000; $i++) {
	$response = curl_exec($curl);
	$err = curl_error($curl);
	if ($err) {
		echo "cURL Error #:" . $err;
	}
	$myfile = fopen("newfile.html", "w") or die("Unable to open file!");
	fwrite($myfile, $response);
	fclose($myfile);
}
curl_close($curl);
$ru = getrusage();
$time_end = microtime(true);

$execution_time = ($time_end - $time_start);
echo "This process used " . rutime($ru, $rustart, "utime") .
    " ms for its computations\n";
echo "It spent " . rutime($ru, $rustart, "stime") .
    " ms in system calls\n";
echo '<b>Total Execution Time:</b> '.$execution_time.' Mins';
