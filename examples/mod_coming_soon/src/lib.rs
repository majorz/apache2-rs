#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(coming_soon_handler, c_coming_soon_handler, coming_soon_module, b"mod_coming_soon\0");


macro_rules! html_template {() => ("<!doctype html>
<html>
   <head>
      <meta charset=\"utf-8\">
      <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge,chrome=1\">
      <title>PolyDraw / Coming Soon</title>
      <meta name=\"description\" content=\"PolyDraw website is coming soon\">
      <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
   </head>

   <style>
      {}
   </style>

   <body>
      <main>
         <h1>PolyDraw</h1>
         <h2>Open source software development</h2>

         <div class=\"coming-soon\">COMING SOON</div>

         <div class=\"in\">in</div>

         <div class=\"timer\">
            <div class=\"number\">26</div>
            <div class=\"days\">days</div>
         </div>

         <div class=\"urgent-call\">
            <div class=\"description\">
               <div class=\"first\">
                  In the meantime, please sign the international petition against
               </div>

               <div class=\"second\">LIVE ORGAN HARVESTING</div>

               <div class=\"third\">from Falun Gong practitioners in China!</div>
            </div>


            <div class=\"petition\">
               <a href=\"https://www.dafoh.org/petition-to-the-united-nations/\" class=\"button\" target=\"_blank\">
                  JOIN THE PETITION!
               </a>
            </div>
         </div>
      </main>

      <footer>
         <div class=\"social\">
            <a href=\"https://www.facebook.com/\" class=\"facebook\" target=\"_blank\"></a>

            <a href=\"https://twitter.com/polydraw\" class=\"twitter\" target=\"_blank\"></a>

            <a href=\"https://github.com/polydraw\" class=\"github\" target=\"_blank\"></a>
         </div>
      </footer>
   </body>
</html>
")}


fn coming_soon_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "coming-soon" || try!(r.uri()) != "/" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/html");

   try!(r.write(format!(
      html_template!(),
      STYLESHEET
   )));

   Ok(Status::OK)
}


const STYLESHEET: &'static str = "
   @font-face {
      font-family: 'Fira Sans';
      font-style: normal;
      font-weight: 300;
      src: local('Fira Sans Light'), local('FiraSans-Light'), url(https://fonts.gstatic.com/s/firasans/v5/VTBnrK42EiOBncVyQXZ7j-gdm0LZdjqr5-oayXSOefg.woff2) format('woff2');
      unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2212, U+2215, U+E0FF, U+EFFD, U+F000;
   }

   @font-face {
      font-family: 'Fira Sans';
      font-style: normal;
      font-weight: 500;
      src: local('Fira Sans Medium'), local('FiraSans-Medium'), url(https://fonts.gstatic.com/s/firasans/v5/zM2u8V3CuPVwAAXFQcDi4Ogdm0LZdjqr5-oayXSOefg.woff2) format('woff2');
      unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2212, U+2215, U+E0FF, U+EFFD, U+F000;
   }

   @font-face {
      font-family: 'Roboto Condensed';
      font-style: normal;
      font-weight: 400;
      src: local('Roboto Condensed'), local('RobotoCondensed-Regular'), url(https://fonts.gstatic.com/s/robotocondensed/v13/Zd2E9abXLFGSr9G3YK2MsDAdhzWOYhqHvOZMRGaEyPo.woff2) format('woff2');
      unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2212, U+2215, U+E0FF, U+EFFD, U+F000;
   }

   @font-face {
      font-family: 'Khand';
      font-style: normal;
      font-weight: 400;
      src: local('Khand'), local('Khand-Regular'), url(https://fonts.gstatic.com/s/khand/v4/jE6debCT41WQse1Htsii-w.woff2) format('woff2');
      unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2212, U+2215, U+E0FF, U+EFFD, U+F000;
   }

   html, body {
      height: 100%;
      width: 100%;
      margin: 0;
   }

   body {
      display: block;
      font-family: 'Roboto Condensed', sans-serif;
      background-image: url(https://www.polydraw.com/img/mountain-mist.jpg);
      background-size: cover;
      background-repeat: no-repeat;
      background-attachment: fixed;
      background-position: right bottom;
   }

   main {
      display: block;
      position: relative;
      top: 50%;
      margin: 0 auto;
      -webkit-transform: translateY(-50%);
      -ms-transform: translateY(-50%);
      transform: translateY(-50%);
   }

   h1 {
      display: block;
      width: 60%;
      margin: 0 auto;
      font-family: 'Fira Sans', sans-serif;
      font-weight: 500;
      font-size: 3.5em;
      color: white;
      text-align: center;
   }

   h2 {
      display: block;
      width: 70%;
      margin: -0.2em auto 0;
      font-family: 'Fira Sans', sans-serif;
      font-weight: 300;
      font-size: 1.2em;
      color: #5b5b5b;
      text-align: center;
   }

   .coming-soon {
      display: block;
      width: 40%;
      margin: 2.5em auto .1em;
      font-size: 1.5em;
      color: white;
      text-align: center;
   }

   .in {
      display: block;
      font-family: 'Fira Sans', sans-serif;
      font-weight: 300;
      font-size: .8em;
      font-style: italic;
      color: white;
      text-align: center;
   }

   .timer {
      display: block;
      width: 7em;
      text-align: center;
      border-style: solid none solid none;
      border-color: #e6e6e6;
      border-width: 1px;
      margin: 0 auto 1em;
   }

   .timer .number {
      display: block;
      font-family: 'Khand', sans-serif;
      font-size: 4em;
      font-weight: bold;
      color: white;
      text-align: center;
   }

   .timer .days {
      display: block;
      font-family: 'Khand', sans-serif;
      font-size: 1.5em;
      color: white;
      text-align: center;
      margin: -1em auto 0;
   }

   .urgent-call {
      margin: 3% auto 0;
   }

   .urgent-call .description {
      display: block;
      width: 100%;
      padding-top: 1em;
      padding-bottom: 1em;
      background: -webkit-linear-gradient(left, rgba(86,86,107,0), rgba(86,86,107,.4), rgba(86,86,107,0));
      background: linear-gradient(to right,rgba(86,86,107,0),rgba(86,86,107,.4),rgba(86,86,107,0));
   }

   .urgent-call .first, .urgent-call .second, .urgent-call .third {
      margin: 0 auto;
      width: 90%;
      color: white;
      font-size: 1em;
      text-align: center;
   }

   .urgent-call .first {
      margin-bottom: 0.5em;
   }

   .urgent-call .second {
      width: 100%;
      color: #f36c69;
      font-family: 'Khand', sans-serif;
      font-weight: bold;
      font-size: 1.2em;
      text-shadow: 0px 1px 9px rgba(0, 0, 0, 0.35);
      letter-spacing: 0.1em;
   }

   .urgent-call .third {
      margin-top: 0.15em;
   }

   .petition .button {
      display: block;
      width: 120px;
      margin: 1em auto 0;
      padding: 3px 6px;
      background: #829776;
      color: #ffffff;
      font-family: 'Fira Sans', sans-serif;
      font-size: .85em;
      font-weight: 300;
      text-align: center;
      text-decoration: none;
      border: solid #ffffff 1px;
      border-radius: 3px;
   }

   .petition .button:hover {
      background: #4d6b3c;
   }


   footer {
      display: table;
      position: fixed;
      width: 96%;
      bottom: 0;
      padding-left: 2%;
      padding-right: 2%;
   }

   .social {
      display: block;
      vertical-align: middle;
      float: right;
   }

   .social a {
      display: inline-block;
      width: 28px;
      height: 28px;
      background-repeat: no-repeat;
      opacity: .5;
      margin-left: .5em;
   }

   .social a:hover {
      opacity: .9;
   }

   .social a.facebook {
      background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAMAAABF0y+mAAAAG1BMVEUAAADY2NjY2NjY2NjY2NhGRkZqamqFhYWtra1jMrL/AAAACXRSTlMAPGaw//////9HdVBWAAAAp0lEQVQoz42TQRLDIAwDVyb/f3GxegAGmJC0PhkWge0RYoYQxuvGSGKkTu9wohW3vQj2yAQoR4Zwh3fWaYAODEJwQWNltqAKEJXShSV71dVWGJCJLuy6+MxFEKwN8uGSPGraml+uaG9s9RuqvVX8FLvSgqJ5PPysdPASP5SP1AS5O2IuEkHRcfCuCFSOt1Y3J5wGkW5O8IFmDg/daTPYH9Z8N/X5O3wBpdI+PxtA6DIAAAAASUVORK5CYII=);
   }

   .social a.twitter {
      background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAMAAABF0y+mAAAAJFBMVEUAAADY2NjV1dXY2NjY2NjY2NjY2NhHR0empqaQkJBtbW3Dw8PQfCyMAAAADHRSTlMAPPVmwKD////////J66Z0AAAA5UlEQVQoz32T0ZKFIAxDT1qc/f+/3VFo9wFQXL23D+oQGpJQxVXCCHJdmB+lFQCq17yDclsoonXYx9NZ+dPy7PQbBEBro/MFw0gwUPJSKRAUewOJiqEbZu5QAUyITQBbVgAPwHUA5OHQO6N4zOMsAFpKW89lAw8FgLW+6XB1Hz9KcuY6XuEjt12XqBKn2bGwgMcpfe7a/XFRYQ+uUzRY9wdE7/DfCdX0cZp5l2jt5FI46QC5F5M4Fl31e/AOmW9oDbB+c89S9klI7POYkIvf4aJd05do5Y6IZTQhYjqsqvE/yLff4Q+6BFspw2Tr1AAAAABJRU5ErkJggg==);
   }

   .social a.github {
      background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAMAAABF0y+mAAAAJFBMVEUAAADY2NjY2NjY2NjY2NjY2NhHR0ebm5u3t7dYWFh3d3dqampwrbVmAAAADHRSTlMAPGbxsP////////80TrKnAAAA4UlEQVQoz32TwW7DMAxDH+ns/z94tbhDpDRBi+lkSCBN0bR4lxAh98YcVgxAaec51NKNIj1eANjmyR+AA8CLZxmqkR8zwAQMEl9KAsExQy+RXaPqxXEBl9gidnZDowGu0olxvBvqcUTSOgmGKlhudeU6JJWLbmmWXwXs04Ga1UaqVK/z9LsuT9+2/XxxovWkFwB2pjfI7ZaB8L6Q53aFdw44UqZb8dyek0R4WMUa5iybIEvNuv83XqBnDLqqgiH59p7JmYTwCd0TE3KLaHPWO315ZiV1jyZUFLUdl2D99x3+AND/ZE+Y11fiAAAAAElFTkSuQmCC);
   }

   @media all and (max-width: 319px) {
      main {
         margin: auto 0;
      }

      h1 {
         margin: -7% auto 0;
         font-size: 1.6em;
      }

      h2 {
         margin: 0 auto;
         font-size: .8em;
      }

      .coming-soon {
         margin: .7em auto 0;
         font-size: .8em;
      }

      .in {
         font-size: .7em;
      }

      .timer {
         width: 3em;
         margin: 0 auto .2em;
      }

      .timer .number {
         font-size: 2em;
      }

      .timer .days {
         font-size: .75em;
      }

      .urgent-call {
         margin: 0 auto;
      }

      .urgent-call .description {
         font-size: 0.7em;
      }

      .petition .button {
         width: 120px;
         margin: 1em auto 0;
         padding: 3px;
         font-size: .6em;
      }
   }

   @media all and (min-width: 320px) and (max-width: 460px) {
      h1 {
         margin: 0 auto;
         font-size: 2.5em;
      }

      h2 {
         margin: 0 auto 2em;
         font-size: .9em;
      }

      .coming-soon {
         margin: 0 auto;
         font-size: 1.3em;
      }

      .timer {
         width: 5em;
         margin: 0 auto 1em;
      }

      .timer .number {
         font-size: 2.8em;
      }

      .timer .days {
         font-size: 1.1em;
      }

      .urgent-call .description {
         font-size: .85em;
      }

      .petition .button {
         width: 100px;
         margin: 1em auto 0;
         font-size: .6em;
      }
   }

   @media all and (min-width: 1600px) {
      h1 {
         font-size: 5em;
      }

      h2 {
         font-size: 1.7em;
      }

      .coming-soon {
         font-size: 2.25em;
      }

      .in {
         font-size: 1.2em;
      }

      .timer {
         width: 10em;
         margin: 0.5em auto 2em;
      }

      .timer .number {
         font-size: 5em;
      }

      .timer .days {
         font-size: 2.25em;
      }

      .urgent-call .description {
      }

      .urgent-call .first, .urgent-call .third {
         font-size: 1.5em;
      }

      .urgent-call .second {
         font-size: 1.8em;
      }

      .petition .button {
         width: 180px;
         margin: 1em auto 0;
         padding: 5px;
         font-size: 1.2em;
      }
   }
";
