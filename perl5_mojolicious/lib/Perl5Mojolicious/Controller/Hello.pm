package Perl5Mojolicious::Controller::Hello;
use Mojo::Base 'Mojolicious::Controller';

sub world {
  my $self = shift;
  $self->render(what => 'world');
}

1;
