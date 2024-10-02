<?php include('header.php'); ?>
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css" />

<?php
$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

$conn = new mysqli($servername, $username, $password, $dbname);
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
if ($_SERVER["REQUEST_METHOD"] == "GET") {
    $debug = trim($_GET['debug'] ?? '');
}
function Debug(){show_source(__FILE__);}
$cur_username = $_SESSION['username'];
$posts = $conn->prepare("SELECT post_id, latitude, longitude, description, descriptor FROM posts WHERE public=1 or username = ?");
$posts->bind_param("s", $cur_username);
$posts->execute();
$result = $posts->get_result();


$points = [];
if ($result->num_rows > 0) {
  while ($row = $result->fetch_assoc()) {
        $comments = $conn->prepare("SELECT * FROM comments WHERE post_id = ? ORDER BY comment_id ASC");
        $comments->bind_param("i", $row['post_id']);
        $comments->execute();
        $re = $comments->get_result();
     

        $content = '<div class="content">';
        $content .= '<h1>' . htmlspecialchars($row['descriptor']) . '</h1>';
        #$content .= '<a><img src="' . htmlspecialchars($row['pic_location']) . '"></a>';
        $content .= '<p>' . htmlspecialchars($row['description']) . '</p>';
        $content .= '</div>';
        #$content .= '<div class"comments">';
        $content .= '<h3> Comments:</h3>';
        if ($re->num_rows>0){
          $content .= '<div class="comment-container">';
          while ($com = $re->fetch_assoc()){
              $content .= '<div class"comment">';
              $content .= '<div class="author" style="font-weight: bold; color: #333; font-size: 1.1em;">' . htmlspecialchars($com['username']) . ':</div>';
              $content .= '<div class="content" style="margin-top: 5px; color: #555; font-size: 1em;"> ' . htmlspecialchars($com['comment']) . '</div> <br>';
              $content .= '</div>';
          }
          $content .= '</div>';
        } else {
          $content .= 'No Comments yet';
        }
        if ($cur_username !== null){
          $content .= '<h4> Write a comment </h4>';
                $content .= '<form method="post" action="submit_comment.php" class="comment-form">';
        $content .= '<label for="comment">Comment:</label>';
        $content .= '<textarea id="comment" name="comment" rows="4" required></textarea>';
        $content .= '<input type="hidden" id="post_id" name="post_id" value="' . htmlspecialchars($row['post_id']) . '">';

        $content .= '<input type="submit" value="Submit">';
        $content .= ' </form>';

        }


        $content .= '</div>';
        $points[] = [
            'popup' => htmlspecialchars($row['descriptor']),
            'lat' => (float) $row['latitude'],
            'lng' => (float) $row['longitude'],
            'link' => '#' . $row['post_id'],
            #'image' => htmlspecialchars($row['pic_location']),
            'description' => htmlspecialchars($row['description']),
            'content' => htmlspecialchars($content, ENT_QUOTES, 'UTF-8')
        ];
    }
} 
$conn->close();

$points_json = json_encode($points, JSON_HEX_TAG | JSON_HEX_AMP | JSON_HEX_APOS | JSON_HEX_QUOT);
?>


    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css" />

    <div id="map"></div>
    <div id="sidebar">
        <div id="close-btn">&times;</div>
        <div id="sidebar-content"></div>
    </div>

    <script src="https://unpkg.com/leaflet@1.7.1/dist/leaflet.js"></script>
    <script>
        var map = L.map('map').setView([51.505, -0.09], 8);

        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(map);

        var points = <?php echo $points_json; ?>;

        points.forEach(function(point) {
            var marker = L.marker([point.lat, point.lng]).addTo(map);
            marker.bindPopup('<b>' + point.popup + '</b><br><a href="'+ point.link +'" onclick="showSidebar(\'' + point.content + '\')">Read more</a>');
        });

        window.onload = function() {
            const hash = window.location.hash;
            if (hash) {
                for (let point of points) {
                    if (point.link === hash) {
                        showSidebar(point.content);
                        break;
                    }
                }
            }
        };

        function showSidebar(content) {
            document.getElementById('sidebar-content').innerHTML = decodeHtml(content);
            document.getElementById('sidebar').classList.add('active');
            document.getElementById('map').classList.add('shrink');
            document.getElementById('header').classList.add('shrink');
        }

        document.getElementById('close-btn').onclick = function() {
            document.getElementById('sidebar').classList.remove('active');
            document.getElementById('map').classList.remove('shrink');
            document.getElementById('header').classList.remove('shrink');
        }

        function decodeHtml(html) {
            var txt = document.createElement("textarea");
            txt.innerHTML = html;
            return txt.value;
        }
    </script>

<?php include('footer.php'); ?>

