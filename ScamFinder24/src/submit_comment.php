<?php
session_start();

$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

$conn = new mysqli($servername, $username, $password, $dbname);

if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_SESSION['username']) && isset($_POST['comment'])) {
    $username = $_SESSION['username'];
    $comment = filter_input(INPUT_POST, 'comment', FILTER_SANITIZE_STRING);
    $post_id = $_POST['post_id'];
    
    $stmt = $conn->prepare("INSERT INTO comments (username, comment, post_id) VALUES (?, ?, ?)");
    $stmt->bind_param("ssd", $username, $comment, $post_id); 
    $stmt->execute();
    $stmt->close();
    $conn->close();
    header("Location: /index.php?#" . $post_id);
}
?>

