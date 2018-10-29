<!DOCTYPE html>
<html>

<head>
  <!-- Standard Meta -->
  <meta charset="utf-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0">

  <!-- Site Properties -->
  <title>@yield('title')</title>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/semantic-ui/2.2.4/semantic.min.css">


  @yield('css')
  <style type="text/css">
    .hidden.menu {
      display: none;
    }

    .masthead.segment {
      min-height: 700px;
      padding: 1em 0em;
    }

    .masthead .logo.item img {
      margin-right: 1em;
    }

    .masthead .ui.menu .ui.button {
      margin-left: 0.5em;
    }

    .masthead h1.ui.header {
      margin-top: 3em;
      margin-bottom: 0em;
      font-size: 4em;
      font-weight: normal;
    }

    .masthead h2 {
      font-size: 1.7em;
      font-weight: normal;
    }

    .ui.vertical.stripe {
      padding: 8em 0em;
    }

    .ui.vertical.stripe h3 {
      font-size: 2em;
    }

    .ui.vertical.stripe .button+h3,
    .ui.vertical.stripe p+h3 {
      margin-top: 3em;
    }

    .ui.vertical.stripe .floated.image {
      clear: both;
    }

    .ui.vertical.stripe p {
      font-size: 1.33em;
    }

    .ui.vertical.stripe .horizontal.divider {
      margin: 3em 0em;
    }

    .quote.stripe.segment {
      padding: 0em;
    }

    .quote.stripe.segment .grid .column {
      padding-top: 5em;
      padding-bottom: 5em;
    }

    .footer.segment {
      padding: 5em 0em;
    }

    .secondary.pointing.menu .toc.item {
      display: none;
    }

    @media only screen and (max-width: 700px) {
      .ui.fixed.menu {
        display: none !important;
      }

      .secondary.pointing.menu .item,
      .secondary.pointing.menu .menu {
        display: none;
      }

      .secondary.pointing.menu .toc.item {
        display: block;
      }

      .masthead.segment {
        min-height: 350px;
      }

      .masthead h1.ui.header {
        font-size: 2em;
        margin-top: 1.5em;
      }

      .masthead h2 {
        margin-top: 0.5em;
        font-size: 1.5em;
      }
    }
  </style>
  @yield('js')
  <script src="https://cdn.bootcss.com/jquery/3.3.1/jquery.min.js"></script>
  <script src="https://cdn.jsdelivr.net/semantic-ui/2.2.4/semantic.min.js"></script>
  <script>
    $(document)
      .ready(function () {
        // fix menu when passed
        $('.masthead')
          .visibility({
            once: false,
            onBottomPassed: function () {
              $('.fixed.menu').transition('fade in');
            },
            onBottomPassedReverse: function () {
              $('.fixed.menu').transition('fade out');
            }
          })
          ;
        // create sidebar and attach to menu open
        $('.ui.sidebar')
          .sidebar('attach events', '.toc.item')
          ;
      })
      ;
  </script>

</head>

<body>
  <!-- Page Contents -->
  <div class="ui vertical masthead center aligned segment">

    <div class="ui container">
      <div class="ui large secondary menu">
        <a class="item">
          <img class="ui mini circular image" src="/logo/gitcube_small.png">
        </a>
        <div class="item">
          <div class="ui icon input">
            <input type="text" placeholder="Search...">
            <i class="search link icon"></i>
          </div>
        </div>
        <a class="active item">Home</a>
        <a class="item">Explore</a>
        <a class="item">Help</a>
        <div class="right item">
          <a class="ui primary button">Sign in</a>
          <a class="ui button">Sign Up</a>
        </div>
      </div>
    </div>


    @yield('content')

    <div class="ui vertical footer segment">
      <div class="ui container">
        <div class="ui stackable inverted divided equal height stackable grid">
          <div class="seven wide column">
            <div class="ui horizontal list">
              <div class="disabled item" href="#">© 2018 GitCube, Inc.</div>
              <a class="item" href="#">Terms</a>
              <a class="item" href="#">Privacy</a>
              <a class="item" href="#">Contact</a>
            </div>
          </div>
          <div class="two wide column">
            <a href="#">
              <img class="ui mini circular image centered" src="/logo/gitcube_small.png">
            </a>
          </div>
          <div class="seven wide column">
            <div class="ui right floated horizontal list">
                <a class="item" href="#">Status</a>
              <a class="item" href="#">Help</a>
              <a class="item" href="#">Contact</a>
              <a class="item" href="#">API</a>
              <a class="item" href="#">About Us</a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

</body>

</html>