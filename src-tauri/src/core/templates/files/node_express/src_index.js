const express = require('express');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

app.get('/health', (req, res) => {
  res.json({ status: 'ok', name: '{{project_name}}' });
});

app.listen(PORT, () => {
  console.log(`{{project_name}} running on http://localhost:${PORT}`);
});
