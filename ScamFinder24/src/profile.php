<?php include('header.php');?>
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
if ($_SERVER["REQUEST_METHOD"] == "GET") {
    $debug = trim($_GET["debug"] ?? '');
}
if (isset($_SESSION['username'])) {
    $cur_username = $_SESSION['username'];
}else{
    $cur_username = null;
}
?>
<div class="login-container">
<?php if (isset($cur_username)): ?>
  

  <h1>Profile of "<?php echo htmlspecialchars($cur_username);?>"</h1>
  <br><br>
  <div> <h3>posts created: <?php 
$posts = $conn->prepare("SELECT count(*) FROM posts where username = ?");
$posts->bind_param("s", $cur_username);
$posts->execute();
$posts->bind_result($post_count);
$posts->fetch();
echo htmlspecialchars($post_count);
$posts->close();
?> </h3> </div>

<?php
$posts = $conn->prepare("SELECT post_id, descriptor FROM posts where username = ?");
$posts->bind_param("s", $cur_username);
$posts->execute();
$result = $posts->get_result();
$posts->close();

if ($result->num_rows > 0) {
    echo "<h3>Your posts:</h3>";
    while ($row = $result->fetch_assoc()) {
        echo "<a href='index.php?#" . htmlspecialchars($row["post_id"]) . "'>" . htmlspecialchars($row["descriptor"]) . "</a><br>";
    }

}

?>


 <div> <h3>comments created: <?php 

$comments = $conn->prepare("SELECT count(*) FROM comments where username = ?");
$comments->bind_param("s", $cur_username);
$comments->execute();
$comments->bind_result($comment_count);
$comments->fetch();

$comments->close();
echo htmlspecialchars($comment_count);?> </h3> </div>

  

<?php else: ?>
<h1>Login to see your profile</h1>

<?php endif; ?>

</div>
<?php include('footer.php'); ?>

