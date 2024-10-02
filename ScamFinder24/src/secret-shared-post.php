<?php include('header.php');
?>
<?php session_start();?>
<?php
$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

$conn = new mysqli($servername, $username, $password, $dbname);

if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
function Debug(){show_source(__FILE__);}
if ($_SERVER['REQUEST_METHOD'] === 'GET') {
    $debug = trim($_GET['debug'] ?? '');
}
?>


<?php if ($_SERVER['REQUEST_METHOD'] === 'GET'): ?>
<div class="login-container">
    <h1> Share a Secret Post</h1>
    <div class="login-form2" id="post-form">
        <h2> Create a Secret Post </h2>
        <form method="post" action="">
            <label for="loc">Location (lat, lon):</label><br>
            <input type="text" id="latitude" name="loc" placeholder="location lat,lon" required><br>
            <label for="descriptor">Title:</label><br>
            <input type="text" id="descriptor" name="descriptor" placeholder="title" required><br>
            Description: <textarea id="description" name="description" rows="4" placeholder="write your description here" required></textarea>
            <button type="submit">Submit</button>
        </form>
    </div>
    <h2> Open a Secret Post </h2>
    <div class="login-form2">
        <form method="post" id="myForm" action="">
            <label for="descriptor">ID:</label><br>
            <input type="text" id="id" name="id" placeholder="id" required><br>
            <label for="location_pre">Location (lat, lon):</label><br>
            <input type="text" id="location_pre" name="location_pre" placeholder="location lat,lon" required><br>
<input type="hidden" id="location" name="location">
            <button type="submit">Submit</button>
        </form>
        <script>
            document.getElementById('myForm').addEventListener('submit', function(event) {
                event.preventDefault();
                const pointValue = document.getElementById('location_pre').value;
                const polygonValue = create_polygon_string(pointValue);
                document.getElementById('location').value = polygonValue;
                this.submit();              // TODO gives Internal error 500 atm
            });

            function create_polygon_string(pointValue) {
                const [lat, lon] = pointValue.split(',');


                let latitude = parseFloat(lat);
                let longitude = parseFloat(lon);

                let polygonString = '';
                polygonString += `${latitude + 0.0001},${longitude + 0.0001};`;
                polygonString += `${latitude - 0.0001},${longitude + 0.0001};`;
                polygonString += `${latitude - 0.0001},${longitude - 0.0001};`;
                polygonString += `${latitude + 0.0001},${longitude - 0.0001};`;
                polygonString += `${latitude + 0.0001},${longitude + 0.0001}`;




                return polygonString;
            }

        </script>
    </div>
 </div>
<?php endif; ?>

<?php
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $debug = trim($_POST['debug'] ?? '');

    function edit_input($input) {
        $pairs = explode(";", $input);
        $result = array();

        foreach ($pairs as $pair) {
            list($x, $y) = explode(",", $pair);
            $result[] = array((float)$x, (float)$y);
        }
        return $result;
    }
    function edit_checkpoint($input) {
         $result = array();
         list($x, $y) = explode(",", $input);
         $result = array((float)$x, (float)$y);

        return $result;
    }

    if (isset($_POST['location']) && isset($_POST['id'])) {
        $location = $_POST['location'];


        $id = (int)$_POST['id'];

        $posts = $conn->prepare("SELECT location as loc FROM secretPosts WHERE post_id = ?");

        if ($posts === false) {
            die("Prepare failed: " . $conn->error);
        }

        $posts->bind_param("i", $id); // "i" for integer
        $posts->execute();
        $result = $posts->get_result();
        if ($result === false) {
            die("Execute failed: " . $posts->error);
        }
        $posts->bind_result($loc);
        $posts->execute();

        if ($posts->fetch()) {
            $local_point = $loc;
        }else {
            $local_point = null;
        }
        $payload =json_encode([
            "searchpoints" => edit_input($location),
            "check_point" => edit_checkpoint($local_point)
            ]
        );

        $curl = curl_init();
        if (!$curl) {
            echo "Failed to initialize cURL\n";
            exit;
        }

        $options = array(
            CURLOPT_URL => "http://api:6223/check-point",
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_TIMEOUT => 30,
            CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
            CURLOPT_CUSTOMREQUEST => "POST",
            CURLOPT_POSTFIELDS => $payload,
            CURLOPT_HTTPHEADER => array(
                "Content-Type: application/json",
                "Content-Length: " . strlen($payload)
            ),
        );

        curl_setopt_array($curl, $options);
        $response = curl_exec($curl);

        $err = curl_error($curl);

        curl_close($curl);

        if ($err) {
            echo "cURL Error #:" . $err;
        }else {

            $responseData = json_decode($response, true);



            if (!isset($responseData['close'])) {
                echo '<div class="login-container"> denied </div>';
            } elseif ($responseData['close'] == '0') {
                echo '<div class="login-container"> denied </div>';
            } elseif ($responseData['close'] == '1'){
                $posts = $conn->prepare("SELECT location as loc                                   FROM secretPosts WHERE post_id = ?");
                $posts = $conn->prepare("SELECT post_id, location as loc, description, descriptor FROM secretPosts WHERE post_id = ?");
                $posts->bind_param("s", $id);
                $posts->execute();
                $result = $posts->get_result();

                $points = [];


                if ($result->num_rows > 0) {
                    while ($row = $result->fetch_assoc()) {
                        $loc_post = edit_checkpoint($row['loc']);
                        $content = '<div class="content">';
                        $content .= '<h1>' . htmlspecialchars($row['descriptor']) . '</h1>';
                        #$content .= '<a><img src="' . htmlspecialchars($row['pic_location']) . '"></a>';
                        $content .= '<p>' . htmlspecialchars($row['description']) . '</p>';
                        $content .= '</div>';

                        $points[] = [
                            'popup' => htmlspecialchars($row['descriptor']),
                            'lat' => (float) $loc_post[0],
                            'lng' => (float) $loc_post[1],
                            'link' => '#' . $row['post_id'],
                            'description' => htmlspecialchars($row['description']),
                            'content' => htmlspecialchars($content, ENT_QUOTES, 'UTF-8')
                        ];
                    }
                }
                #echo json_encode($points);
                $conn->close();

                $points_json = json_encode($points, JSON_HEX_TAG | JSON_HEX_AMP | JSON_HEX_APOS | JSON_HEX_QUOT);
                $template = file_get_contents('maptemplate.html');
                $output = str_replace('$points_json', $points_json, $template);

                $output = str_replace('$lat_preview', $points[0]['lat'], $output);
                $output = str_replace('$lng_preview', $points[0]['lng'], $output);
                echo $output;
            }
        }
    } elseif (isset($_POST['loc']) && isset($_POST['descriptor']) && isset($_POST['description'])) {
        $current_username = $_SESSION['username'] ?? null;

        $location = $_POST['loc']; // TODO: Sanitize input for float,float in a string.
        $descriptor = filter_input(INPUT_POST, 'descriptor', FILTER_SANITIZE_STRING);
        $description = filter_input(INPUT_POST, 'description', FILTER_SANITIZE_STRING);

        # on workflow somehow fails here: Fatal error:  Uncaught Error: Call to a member function bind_param() on bool in /var/www/html/secret-shared-post.php:217
        $stmt = $conn->prepare("INSERT INTO secretPosts (location, description, descriptor, username) VALUES (?, ?, ?, ?)");
        $stmt->bind_param("ssss", $location, $description, $descriptor, $current_username);


        $stmt->execute();
        $stmt->close();

        $posts = $conn->prepare("SELECT post_id FROM secretPosts WHERE location = ? and description = ? and descriptor = ? ORDER BY creation_time DESC LIMIT 1");

        $posts->bind_param("sss", $location, $description, $descriptor);
        $posts->execute();

        $posts->bind_result($post_id);


        // Fetch the result and store it in a variable
        if ($posts->fetch()) {
            $post_id = $post_id;
        }else {
          $post_id = null;
        }

          echo '<div class="login-container">' . htmlspecialchars($descriptor) .  " has been posted! the post id is: " . htmlspecialchars($post_id) . " </div>";

    }
}
?>
<?php include('footer.php'); ?>
