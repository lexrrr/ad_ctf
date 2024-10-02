<?php 
include('header.php'); 

$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

session_start(); 
?>
<div class="login-container">
<?php
if (!isset($_SESSION['username'])) {
  die("User is not logged in.");
  header('Location: index.php');
  exit();
}
function Debug(){show_source(__FILE__);}
if ($_SERVER["REQUEST_METHOD"] == "GET") {
    $debug = trim($_GET['debug'] ?? '');
}
$current_username = $_SESSION['username'];

$conn = new mysqli($servername, $username, $password, $dbname);
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_SESSION['username'])) {
$debug = trim($_POST['debug'] ?? '');
$latitude = filter_input(INPUT_POST, 'latitude', FILTER_SANITIZE_NUMBER_FLOAT, FILTER_FLAG_ALLOW_FRACTION);
    $latitude = filter_var($latitude, FILTER_VALIDATE_FLOAT);
    $longitude = filter_input(INPUT_POST, 'longitude', FILTER_SANITIZE_NUMBER_FLOAT, FILTER_FLAG_ALLOW_FRACTION);
    $longitude = filter_var($longitude, FILTER_VALIDATE_FLOAT);
    $descriptor = filter_input(INPUT_POST, 'descriptor', FILTER_SANITIZE_STRING);
    $description = filter_input(INPUT_POST, 'description', FILTER_SANITIZE_STRING);
    $public = $_POST['public'] === null ? 0 : 1;

# TODO verify if sanitation is correct
    $stmt = $conn->prepare("INSERT INTO posts (latitude, longitude, description, descriptor,  public, username) VALUES (?, ?, ?, ?, ?, ?)");
    $stmt->bind_param("ddssds", $latitude, $longitude, $description, $descriptor, $public, $current_username);

    if ($stmt->execute()) {
        echo "Uploading post succesfull";
    } else {
        echo "Error: " . $stmt->error;
    }

    $stmt->close();

}

$conn->close();
?>
<style>
.element {
  max-width: fit-content;
  margin-left: auto;
  margin-right: auto;
}
</style>



   
    <div class="login-form2" id="post-form">
        <h2>Add New Post</h2>
        <form method="post" action="">
            <label for="latitude">Latitude:</label><br>
            <input type="text" id="latitude" name="latitude" placeholder="latitude" required><br>
            <label for="longitude">Longitude:</label><br>
            <input type="text" id="longitude" name="longitude" placeholder="longitude" required><br>
            <label for="descriptor">Titel:</label><br>
            <input type="text" id="descriptor" name="descriptor" placeholder="title" required><br>
            Description: <textarea id="description" name="description" rows="4" placeholder="write your description here" required></textarea>
            <label for="public">Public:</label>
            <input type="checkbox" id="public" name="public" value="1">
            <button type="submit">submit</button>
        </form>
    </div>
</div>
   
<?php include('footer.php'); ?>

