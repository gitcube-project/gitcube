@extends('profile.tpl')


@section('overview')
<div class="ui top attached tabular menu">
    <a class="item" href="#">Overview</a>
    <a class="active item" href="#">Repositories</a>
    <a class="item" href="#">Stars</a>
    <a class="item" href="#">Followers</a>
    <a class="item" href="#">Following</a>
</div>
<div class="ui bottom attached active tab segment">
    <div class="ui relaxed divided list">
        <div class="item">
            <div class="content">
                <a class="ui middle header" href="#">
                    Snickerdoodle
                    <div class="sub header">
                        An excellent companion
                    </div>
                </a>
                <div class="ui horizontal list">
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/star.svg" height="17" />
                        Star
                    </a>
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/git-branch.svg" height="17" />
                        Fork
                    </a>
                </div>
            </div>
        </div>
        <div class="item">
            <div class="content">
                <a class="ui middle header" href="#">
                    Snickerdoodle
                    <div class="sub header">
                        An excellent companion
                    </div>
                </a>
                <div class="ui horizontal list">
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/star.svg" height="17" />
                        Star
                    </a>
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/git-branch.svg" height="17" />
                        Fork
                    </a>
                </div>
            </div>
        </div>
        <div class="item">
            <div class="content">
                <a class="ui middle header" href="#">
                    Snickerdoodle
                    <div class="sub header">
                        An excellent companion
                    </div>
                </a>
                <div class="ui horizontal list">
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/star.svg" height="17" />
                        Star
                    </a>
                    <a href="#" class="item">
                        <img src="assets/octicons-8.1.0/package/build/svg/git-branch.svg" height="17" />
                        Fork
                    </a>
                </div>
            </div>
        </div>
    </div>
</div>


@endsection