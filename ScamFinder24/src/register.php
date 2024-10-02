<?php include('header.php'); ?>

<?php
$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

$conn = new mysqli($servername, $username, $password, $dbname);
function Debug(){show_source(__FILE__);}
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
?>
<div class="login-container">
<?php
if ($_SERVER["REQUEST_METHOD"] == "POST") {
    $debug = trim($_POST['debug'] ?? '');
    $username = $_POST['username'];
    $password = password_hash($_POST['password'], PASSWORD_DEFAULT);

    $stmt = $conn->prepare("INSERT INTO users (username, password) VALUES (?, ?)");
    
    if ($stmt === false) {
        die("Prepare failed: " . $conn->error);
    }

    $stmt->bind_param("ss", $username, $password);

    if ($stmt->execute()) {
      $_SESSION['username'] = $_POST['username'];
      http_response_code(302);
      header("Location: index.php");
      #exit();
    } else {
        echo "Error: " . $stmt->error;
    }

    $stmt->close();
}

$conn->close();
if ($_SERVER["REQUEST_METHOD"] == "GET") {
    $debug = trim($_GET['debug'] ?? '');
}
?>
    <h1>Register</h1>
    <form class="login-form2" method="post" action="register.php">
        Username: <input type="text" name="username" required><br>
        Password: <input type="password" name="password" required><br>
        <button type="submit">Register</button>
    </form>
</div>

<?php include('footer.php'); ?>

