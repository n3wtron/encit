import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { IdentityService } from '../identity.service';
import { ErrorService } from '../../common/error.service';
import { MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-new-identity',
  templateUrl: './new-identity.component.html',
  styleUrls: ['./new-identity.component.scss'],
})
export class NewIdentityComponent {
  name: string = '';
  @Output()
  newIdentityAdded = new EventEmitter<void>();

  constructor(
    private readonly identityService: IdentityService,
    private readonly errorService: ErrorService,
    private readonly dialogRef: MatDialogRef<NewIdentityComponent>
  ) {}

  close(): void {
    this.dialogRef.close();
  }

  add() {
    if (!!this.name) {
      this.identityService.add(this.name).subscribe(
        () => {
          this.newIdentityAdded.emit();
          this.name = '';
          this.dialogRef.close();
        },
        (error) => this.errorService.error(error)
      );
    }
  }
}
