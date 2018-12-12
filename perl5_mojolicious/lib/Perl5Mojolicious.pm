package Perl5Mojolicious;
use Mojo::Base 'Mojolicious';

sub startup {
  my $self = shift;

  my $config = $self->plugin('Config');
  $self->secrets($config->{secrets});

  my $r = $self->routes;

  $r->get('/')->to('hello#world');
}

1;
