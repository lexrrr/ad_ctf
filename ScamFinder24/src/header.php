<?php session_start();?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ScamFinder24</title>
    <!--<link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css" /> -->
    <link rel="stylesheet" href="styles.css" />
    </style>
</head>
<body>
    <div class="container">
        <header id="header">
        <nav>
                <ul>
                    <li><a href="index.php"><h1>ScamFinder24</h1></a></li>
                    <li><a href="index.php">Home</a></li>
                    <li><?php if (isset($_SESSION['username'])): ?>
                        <a href="newpost.php">New Sighting</a>
                            <?php endif; ?>
                    </li>
                    <li><?php if (isset($_SESSION['username'])): ?>
                        <a href="profile.php">Profile</a>
                            <?php endif; ?>
                    </li>
                    <li><a href="secret-shared-post.php">Share Secrete Post</a></li>

                    <li class="login-form">
                        <?php if (isset($_SESSION['username'])): ?>
                <p>Welcome, <?php echo htmlspecialchars($_SESSION['username']); ini_set('display_errors',0);?>! <a href="logout.php">Logout</a></p>
            <?php else: ?>
                <form method="post" action="login.php" style="display: inline;">
                    Login: <input type="text" name="username" placeholder="username" required>
                    <input type="password" name="password" placeholder="password" required>
                    <button type="submit">Login</button>
                </form>
                  </li>
                  <li>
                <a href="register.php">Register</a>
            <?php endif; error_reporting(0); ?>
                    </li>
                </ul>
         </nav>

        </header>
        <main>

