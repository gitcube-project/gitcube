@extends('layouts/main_layout.tpl')

@section('title')
home - GitCube
@endsection

@section('content')
<div class="container">
    <div class="ui grid centered">
        <div class="three wide column">
            <a href="https://avatars0.githubusercontent.com/u/13167278?s=460&v=4" class="ui small image">
                <img src="https://avatars0.githubusercontent.com/u/13167278?s=460&v=4">
            </a>
            <h2 class="ui header">
                Jason Wang
                <div class="sub header">jstzwj</div>
            </h2>
            <h5 class="ui disabled header">
                Learning learning
            </h5>
            <div class="ui divider"></div>
            <div>
                <div class="ui item">
                    <i class="users icon"></i> SooChow University
                </div>
                <div class="ui item">
                    <i class="location arrow icon"></i> SuZhou,China
                </div>
            </div>
            <div class="ui divider"></div>
            <h4 class="ui header">
                Organizations
            </h4>
            <a href="https://github.com/NVIDIAGameWorks" class="ui mini image" data-tooltip="NVIDIAGameWorks" data-inverted="">
                <img src="https://avatars0.githubusercontent.com/u/7717624?s=70&v=4">
            </a>

            <a href="https://github.com/toyteam" class="ui mini image" data-tooltip="toyteam" data-inverted="">
                <img src="https://avatars0.githubusercontent.com/u/22074671?s=70&v=4">
            </a>
        </div>
        
        <div class="ten wide column">
            @yield('overview')
        </div>
    </div>
</div>
@endsection