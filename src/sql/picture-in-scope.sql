select
	count(1) = 1
from
	Picture left join Scope on Picture.scope = Scope.id where Picture.id = $1 and Scope.name = $2
