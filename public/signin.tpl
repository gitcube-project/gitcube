@extends('layouts/main_layout.tpl')

@section('css')
<style type="text/css">
    body {
        background-color: #DADADA;
    }

    body>.grid {
        height: 80%;
    }

    .column {
        max-width: 450px;
    }
</style>
@endsection

@section('content')
<div class="ui middle aligned center aligned grid">
    <div class="column">
        <h2 class="ui teal image header">
            <img src="/logo/gitcube_small.png" class="image">
            <div class="content">
                Sign in to GitCube
            </div>
        </h2>
        <form class="ui large form">
            <div class="ui stacked segment">
                <div class="field">
                    <div class="ui left icon input">
                        <i class="user icon"></i>
                        <input type="text" name="email" placeholder="E-mail address">
                    </div>
                </div>
                <div class="field">
                    <div class="ui left icon input">
                        <i class="lock icon"></i>
                        <input type="password" name="password" placeholder="Password">
                    </div>
                </div>
                <div class="ui fluid large primary submit button">Sign In</div>
            </div>

            <div class="ui error message"></div>

        </form>

        <div class="ui message">
            New to us? <a href="#">Sign Up</a>
        </div>
    </div>
</div>

@endsection