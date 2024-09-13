const express = require("express");
const app = express();

// Import users array from data.js
const users = require("./data");

// Route to search for user by either Username or Name
app.get("/user", async (req, res) => {
  const { username, name } = req.query;

  // If neither username nor name is provided, return 400
  if (!username && !name) {
    return res
      .status(400)
      .send('Please provide either "username" or "name" as a query parameter');
  }

  let user;

  // If username query is provided, search by Username
  if (username) {
    user = users.find(
      (u) => u.Username.toLowerCase() === username.toLowerCase()
    );
  }

  // If name query is provided, search by Name (only if no user found by username)
  if (name && !user) {
    user = users.find((u) => u.Name.toLowerCase() === name.toLowerCase());
  }

  // Respond with user data if found, or return 404 if not found
  if (user) {
    res.json(user);
  } else {
    res.status(404).send("User not found");
  }
});

// Start the server
app.listen(3000, () => {
  console.log("Server is running on http://localhost:3000");
});
