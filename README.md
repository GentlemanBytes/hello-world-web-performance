# Languages, web frameworks, comparison - fun!
Have you ever asked yourself which the fastest language in web is? Well, we did! That's why we want to test this.

## The rules
- Use a language of your choice. You only need to have it at the latest language version.
- Use a web framework of your choice. For example:
  - Symfony for PHP
  - Spring for Java
  - Django for Python
- Write a Hello World App. It needs a full HTML5 compatible markdown tree. To be clear: 
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Hello World!</title>
</head>
<body>
    Hello World!
</body>
</html>
```
- Only one route is defined. It listens to the web root.
- No specified configuration allowed. Use it as it ships!

## Folder structure
The folders within this project each contain a hello world application in a specific language and web framework. The folder names structure is: `$LANGUAGE-$FRAMEWORK`

## Wanting to help?
PRs are truely welcome to get a much wider variety of which language/framework performance best, overall! Just follow the rules and folder structure and we'll propably accept your Pull Request!

## Disclaimer
Yes, we are aware of the fact that these comparisons are not exact. There are more parameters which are important for the performance
of a web application (OS, virtualization, bare metal, application servers, alternative CGI implementations (e.g. php-fpm)...).

This project is useful to compare a simple task as writing a "Hello world!"-website with templating with different languages and frameworks and
how these different implementations differ.

And after all it is a quite fun project.

## License
All code is licensed under the terms and conditions of the [MIT](https://opensource.org/licenses/MIT) license.

## Authors
[GentlemanBytes](https://gentlemanbytes.com) - Steffen Beisenherz ([sironheart](https://github.com/sironheart)) and Alexander Kluth ([deralex](https://github.com/deralex))
