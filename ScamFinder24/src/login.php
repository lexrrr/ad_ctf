<?php

session_start();

$servername = "db";
$username = "user";
$password = "password";
$dbname = "myDB";

$conn = new mysqli($servername, $username, $password, $dbname);
function Debug() {show_source(__FILE__);}
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
if($_SERVER["REQUEST_METHOD"] != "POST"){
  header('Location: index.php'); 
} 
  $password = trim($_POST['password'] ?? '');
  $username = trim($_POST['username'] ?? '');
  $debug = trim($_POST['debug'] ?? '');
  $stmt = $conn->prepare("SELECT id, username, password FROM users WHERE username = ?");
  $stmt->bind_param("s", $username);
  $stmt->execute();
  $stmt->store_result();
  
  if ($stmt->num_rows > 0) {
      $stmt->bind_result($id, $username, $hashed_password);
      $stmt->fetch();

      if (password_verify($password, $hashed_password)) {

        function login(){
          #$_SESSION['username'] = $username;
          $_SESSION['username'] = $_POST['username'];
          http_response_code(302);
          header("Location: index.php");
          exit();
        } 
        login();
      } 
  }
  $stmt->close();
  $conn->close();
  
  include('header.php');
  echo "<div class='login-container'> something went wrong with logging in";
?>
    <h1>Login</h1>
    <form class="login-form2" method="post" action="login.php">
        Username: <input type="text" name="username" placeholder="username" required><br>
        Password: <input type="password" name="password" placeholder="password" required><br>
        <button type="submit">Login</button>
    </form>
</div>

<?php include('footer.php'); ?>
