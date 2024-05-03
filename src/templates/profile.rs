
pub fn profile() -> &'static str{
 PROFILE_TEMPLATE
}


const PROFILE_TEMPLATE: &'static str = r#"
 <!doctype html>
 <html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>A Basic HTML5 Template</title>
  <meta name="description" content="A basic HTML5 Template for new projects.">
  <script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous"></script>

</head>

<body>
    <h1>Profile of {{ profile.full_name|title }}</h1>
    <p>This is a template example to show some functionality</p>
    <h2>Items</h3>
    <ul>
        {% for item in profile.items %}
        <li>{{ item.name }} ({{ item.id }})</li>
        {% endfor %}
    <ul>
    <button>
        Count {{profile.count}}
    </button>
</body>
</html>
"#;
